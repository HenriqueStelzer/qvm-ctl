#!/usr/bin/env bash
# qvm — QEMU/KVM VM manager

set -euo pipefail

QVM_VERSION="1.0.0"

VM_DIR="${QVM_DIR:-$HOME/vms}"
RAM_MB_DEFAULT="${QVM_RAM:-8192}"
VCPUS_DEFAULT="${QVM_CPUS:-4}"
DISK_SIZE_DEFAULT="${QVM_DISK:-40G}"
SPICE_PORT_START=5900

R='\033[0;31m' G='\033[0;32m' Y='\033[1;33m' C='\033[0;36m' B='\033[1m' Z='\033[0m'
info(){ echo -e "${C}→${Z} $*"; }
ok(){   echo -e "${G}✓${Z} $*"; }
warn(){ echo -e "${Y}!${Z} $*"; }
die(){  echo -e "${R}✗${Z} $*" >&2; exit 1; }
sep(){  echo -e "${B}────────────────────────────────────────${Z}"; }

require(){ command -v "$1" &>/dev/null || die "Missing: $1. Install it with your package manager."; }

find_ovmf(){
    local code_paths=(
        /usr/share/OVMF/x64/OVMF_CODE.4m.fd
        /usr/share/edk2/x64/OVMF_CODE.4m.fd
        /usr/share/OVMF/OVMF_CODE.fd
        /usr/share/qemu/OVMF_CODE.fd
    )
    local vars_paths=(
        /usr/share/OVMF/x64/OVMF_VARS.4m.fd
        /usr/share/edk2/x64/OVMF_VARS.4m.fd
        /usr/share/OVMF/OVMF_VARS.fd
        /usr/share/qemu/OVMF_VARS.fd
    )
    OVMF_CODE=""; OVMF_VARS=""
    for p in "${code_paths[@]}"; do [[ -f "$p" ]] && OVMF_CODE="$p" && break; done
    for p in "${vars_paths[@]}"; do [[ -f "$p" ]] && OVMF_VARS="$p" && break; done
    [[ -n "$OVMF_CODE" ]] || die "OVMF_CODE not found. Install: edk2-ovmf (Arch) / ovmf (Debian)"
    [[ -n "$OVMF_VARS" ]] || die "OVMF_VARS not found. Install: edk2-ovmf (Arch) / ovmf (Debian)"
}

vm_pid_file(){  echo "$VM_DIR/$1/$1.pid"; }
vm_log_file(){  echo "$VM_DIR/$1/qemu.log"; }
vm_port_file(){ echo "$VM_DIR/$1/spice.port"; }

vm_is_running(){
    local pidfile; pidfile=$(vm_pid_file "$1")
    [[ -f "$pidfile" ]] || return 1
    local pid; pid=$(cat "$pidfile")
    ps -p "$pid" -o args= 2>/dev/null | grep -q "qemu-system" || return 1
}

next_spice_port(){
    local port=$SPICE_PORT_START
    while ss -Htln "sport = :$port" 2>/dev/null | grep -q .; do ((port++)); done
    echo "$port"
}

load_vm(){
    local file="$1/vm.conf"
    [[ -f "$file" ]] || die "Missing vm.conf in $1"
    local -A _allowed=([NAME]=1 [ISO]=1 [DISK]=1 [OVMF_CODE]=1 [OVMF_VARS]=1 [RAM_MB]=1 [VCPUS]=1 [CREATED]=1)
    while IFS='=' read -r key val; do
        [[ -n "${_allowed[$key]+x}" ]] || continue
        val="${val%\r}"
        printf -v "$key" '%s' "$val"
    done < "$file"
}

cmd_create(){
    local name="${1:-}" iso="${2:-}"
    [[ -n "$name" && -n "$iso" ]] || die "Usage: qvm create <name> <iso>"
    [[ "$name" =~ ^[a-zA-Z0-9][a-zA-Z0-9_-]*$ ]] || die "Invalid VM name '$name'. Use letters, digits, hyphens, underscores. Must not start with '-'."
    [[ -f "$iso" ]] || die "ISO not found: $iso"

    require qemu-system-x86_64
    require qemu-img
    find_ovmf

    local vmdir="$VM_DIR/$name"
    [[ -d "$vmdir" ]] && die "VM '$name' already exists"
    mkdir -p "$vmdir"

    local disk="$vmdir/disk.qcow2"
    qemu-img create -f qcow2 "$disk" "$DISK_SIZE_DEFAULT" -q
    cp "$OVMF_VARS" "$vmdir/ovmf-vars.fd"

    cat > "$vmdir/vm.conf" <<EOF
NAME=$name
ISO=$iso
DISK=$disk
OVMF_CODE=$OVMF_CODE
OVMF_VARS=$vmdir/ovmf-vars.fd
RAM_MB=$RAM_MB_DEFAULT
VCPUS=$VCPUS_DEFAULT
CREATED=$(date -Iseconds)
EOF

    ok "Created VM: $name"
}

cmd_launch(){
    local name="${1:-}" flag="${2:-}"
    [[ -n "$name" ]] || die "Usage: qvm launch <name>"
    [[ -z "$flag" || "$flag" == "--no-iso" ]] || die "Unknown option: '$flag'. Valid: --no-iso"

    local vmdir="$VM_DIR/$name"
    [[ -d "$vmdir" ]] || die "VM '$name' not found. Run: qvm list"
    load_vm "$vmdir"

    local lockfile="$vmdir/.launch.lock"
    exec 9>"$lockfile"
    flock -n 9 || die "VM '$name' is already launching. Try: qvm list"

    if vm_is_running "$name"; then
        local port; port=$(cat "$(vm_port_file "$name")" 2>/dev/null || echo "?")
        warn "Already running → spice://localhost:$port"
        return
    fi

    require qemu-system-x86_64

    local port; port=$(next_spice_port)
    echo "$port" > "$(vm_port_file "$name")"

    local log; log=$(vm_log_file "$name")

    local cmd=(
        qemu-system-x86_64
        -machine q35,accel=kvm
        -cpu host
        -smp "$VCPUS"
        -m "$RAM_MB"
        -drive "if=pflash,format=raw,readonly=on,file=$OVMF_CODE"
        -drive "if=pflash,format=raw,file=$OVMF_VARS"
        -device virtio-gpu-pci
        -spice "port=$port,disable-ticketing=on"
        -device virtio-serial-pci
        -chardev spicevmc,id=vdagent,name=vdagent
        -device virtserialport,chardev=vdagent,name=com.redhat.spice.0
        -nic user,model=virtio-net-pci
        -drive "file=$DISK,if=virtio,format=qcow2"
        -device usb-ehci
        -device usb-tablet
        -display none
        -boot order=dc
    )

    if [[ "$flag" != "--no-iso" && -f "$ISO" ]]; then
        cmd+=( -drive "file=$ISO,media=cdrom,readonly=on" )
        info "Booting from ISO"
    elif [[ "$flag" != "--no-iso" && ! -f "$ISO" ]]; then
        warn "ISO not found at $ISO, falling back to disk"
        info "Booting from disk"
    else
        info "Booting from disk"
    fi

    "${cmd[@]}" >"$log" 2>&1 &
    local pid=$!
    echo "$pid" > "$(vm_pid_file "$name")"

    sleep 1
    vm_is_running "$name" || { rm -f "$(vm_pid_file "$name")"; die "QEMU exited immediately. Check: $log"; }

    ok "Running → PID $pid | spice://localhost:$port"

    if command -v remote-viewer &>/dev/null; then
        remote-viewer "spice://localhost:$port" &
    elif command -v virt-viewer &>/dev/null; then
        virt-viewer --connect "spice://localhost:$port" &
    else
        warn "No SPICE viewer found. Install: virt-viewer"
    fi
}

cmd_stop(){
    local name="${1:-}"
    [[ -n "$name" ]] || die "Usage: qvm stop <name>"

    vm_is_running "$name" || die "VM '$name' is not running"

    local pid; pid=$(cat "$(vm_pid_file "$name")")
    kill -SIGTERM "$pid" || true

    for _ in {1..5}; do
        sleep 1
        vm_is_running "$name" || break
    done

    if vm_is_running "$name"; then
        warn "SIGTERM ignored, sending SIGKILL..."
        kill -SIGKILL "$pid" || true
        sleep 1
    fi

    if vm_is_running "$name"; then
        die "Failed to stop VM '$name'"
    fi

    rm -f "$(vm_pid_file "$name")"
    ok "Stopped $name"
}

cmd_list(){
    sep
    [[ -d "$VM_DIR" ]] || { warn "No VMs found. Run: qvm create <name> <iso>"; sep; return; }

    local found=false
    for d in "$VM_DIR"/*/; do
        [[ -f "$d/vm.conf" ]] || continue
        found=true
        load_vm "$d"

        if vm_is_running "$NAME"; then
            local port; port=$(cat "$d/spice.port" 2>/dev/null || echo "?")
            echo -e "  ${B}$NAME${Z}  ${G}running${Z}  spice://localhost:$port"
        else
            echo -e "  ${B}$NAME${Z}  ${Y}stopped${Z}"
        fi
    done

    [[ "$found" == false ]] && warn "No VMs found. Run: qvm create <name> <iso>"
    sep
}

cmd_disable(){
    local name="${1:-}"
    [[ -n "$name" ]] || die "Usage: qvm disable <name>"

    local vmdir="$VM_DIR/$name"
    [[ -d "$vmdir" ]] || die "VM '$name' not found"

    if vm_is_running "$name"; then
        warn "Stopping running VM..."
        cmd_stop "$name"
    fi

    read -rp "Type '$name' to confirm deletion: " confirm
    [[ "$confirm" == "$name" ]] || die "Aborted"

    rm -rf "$vmdir"
    ok "Deleted $name"
}

cmd_version(){
    echo "qvm $QVM_VERSION"
}

cmd_help(){
    sep
    echo -e "${B}qvm${Z} $QVM_VERSION — QEMU/KVM VM manager"
    sep
    echo -e "  ${C}qvm create  <name> <iso>${Z}     create a new VM"
    echo -e "  ${C}qvm launch  <name>${Z}           boot (with ISO)"
    echo -e "  ${C}qvm launch  <name> --no-iso${Z}  boot from disk"
    echo -e "  ${C}qvm stop    <name>${Z}           gracefully stop a VM"
    echo -e "  ${C}qvm list${Z}                  show all VMs and status"
    echo -e "  ${C}qvm disable <name>${Z}           stop and delete a VM"
    echo -e "  ${C}qvm version${Z}               print version"
    sep
    echo -e "  Environment overrides:"
    echo -e "    QVM_DIR   VM storage directory  (default: ~/vms)"
    echo -e "    QVM_RAM   RAM in MB             (default: 8192)"
    echo -e "    QVM_CPUS  vCPU count            (default: 4)"
    echo -e "    QVM_DISK  Disk size             (default: 40G)"
    sep
}

case "${1:-}" in
    create)       cmd_create  "${2:-}" "${3:-}" ;;
    launch)       cmd_launch  "${2:-}" "${3:-}" ;;
    stop)         cmd_stop    "${2:-}" ;;
    list)         cmd_list ;;
    disable)      cmd_disable "${2:-}" ;;
    version)      cmd_version ;;
    help|--help|-h) cmd_help ;;
    *)            cmd_help ;;
esac
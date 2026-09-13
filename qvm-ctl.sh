#!/usr/bin/env bash

set -euo pipefail

QVM_VERSION="1.1.0"

VM_DIR="${QVM_DIR:-$HOME/vms}"
RAM_MB_DEFAULT="${QVM_RAM:-8192}"
VCPUS_DEFAULT="${QVM_CPUS:-4}"
DISK_SIZE_DEFAULT="${QVM_DISK:-40G}"
SPICE_PORT_START=5900

R='\033[0;31m'
G='\033[0;32m'
Y='\033[1;33m'
C='\033[0;36m'
B='\033[1m'
Z='\033[0m'

info(){ echo -e "${C}→${Z} $*"; }
ok(){   echo -e "${G}✓${Z} $*"; }
warn(){ echo -e "${Y}!${Z} $*"; }
die(){  echo -e "${R}✗${Z} $*" >&2; exit 1; }
sep(){  echo -e "${B}────────────────────────────────────────${Z}"; }

require(){
    command -v "$1" &>/dev/null ||
        die "Missing: $1. Install it with your package manager."
}

find_ovmf(){
    local code_paths=(
        /usr/share/edk2/x64/OVMF_CODE_4M.secboot.fd
        /usr/share/edk2/x64/OVMF_CODE.secboot.fd
        /usr/share/edk2/x64/OVMF_CODE_4M.fd
        /usr/share/edk2/x64/OVMF_CODE.fd
        /usr/share/OVMF/x64/OVMF_CODE.secboot.4m.fd
        /usr/share/OVMF/x64/OVMF_CODE.4m.fd
        /usr/share/OVMF/OVMF_CODE.secboot.fd
        /usr/share/OVMF/OVMF_CODE.fd
        /usr/share/qemu/OVMF_CODE.secboot.fd
        /usr/share/qemu/OVMF_CODE.fd
    )

    local vars_paths=(
        /usr/share/edk2/x64/OVMF_VARS_4M.ms.fd
        /usr/share/edk2/x64/OVMF_VARS.ms.fd
        /usr/share/edk2/x64/OVMF_VARS_4M.fd
        /usr/share/edk2/x64/OVMF_VARS.fd
        /usr/share/OVMF/x64/OVMF_VARS.ms.4m.fd
        /usr/share/OVMF/x64/OVMF_VARS.4m.fd
        /usr/share/OVMF/OVMF_VARS.ms.fd
        /usr/share/OVMF/OVMF_VARS.fd
        /usr/share/qemu/OVMF_VARS.ms.fd
        /usr/share/qemu/OVMF_VARS.fd
    )

    OVMF_CODE=""
    OVMF_VARS=""
    SECURE_BOOT=0

    for p in "${code_paths[@]}"; do
        if [[ -f "$p" ]]; then
            OVMF_CODE="$p"

            [[ "$p" == *secboot* ]] && SECURE_BOOT=1
            break
        fi
    done

    for p in "${vars_paths[@]}"; do
        if [[ -f "$p" ]]; then
            OVMF_VARS="$p"
            break
        fi
    done

    [[ -n "$OVMF_CODE" ]] ||
        die "OVMF_CODE not found. Install: edk2-ovmf"

    [[ -n "$OVMF_VARS" ]] ||
        die "OVMF_VARS not found. Install: edk2-ovmf"
}

vm_dir(){
    echo "$VM_DIR/$1"
}

vm_pid_file(){
    echo "$(vm_dir "$1")/$1.pid"
}

vm_log_file(){
    echo "$(vm_dir "$1")/qemu.log"
}

vm_port_file(){
    echo "$(vm_dir "$1")/spice.port"
}

vm_tpm_pid_file(){
    echo "$(vm_dir "$1")/swtpm.pid"
}

vm_tpm_socket(){
    echo "$(vm_dir "$1")/swtpm.sock"
}

vm_is_running(){
    local name="$1"
    local pidfile
    pidfile=$(vm_pid_file "$name")

    [[ -f "$pidfile" ]] || return 1

    local pid
    pid=$(cat "$pidfile" 2>/dev/null || true)

    [[ "$pid" =~ ^[0-9]+$ ]] || return 1

    if ! kill -0 "$pid" 2>/dev/null; then
        return 1
    fi

    ps -p "$pid" -o comm= 2>/dev/null |
        grep -q '^qemu-system-x86_64$'
}

tpm_is_running(){
    local name="$1"
    local pidfile
    pidfile=$(vm_tpm_pid_file "$name")

    [[ -f "$pidfile" ]] || return 1

    local pid
    pid=$(cat "$pidfile" 2>/dev/null || true)

    [[ "$pid" =~ ^[0-9]+$ ]] || return 1

    kill -0 "$pid" 2>/dev/null
}

cleanup_stale_files(){
    local name="$1"

    if [[ -f "$(vm_pid_file "$name")" ]] &&
       ! vm_is_running "$name"; then
        rm -f "$(vm_pid_file "$name")"
    fi

    if [[ -f "$(vm_tpm_pid_file "$name")" ]] &&
       ! tpm_is_running "$name"; then
        rm -f "$(vm_tpm_pid_file "$name")"
    fi
}

next_spice_port(){
    local port=$SPICE_PORT_START

    while ss -Htln "sport = :$port" 2>/dev/null | grep -q .; do
        ((port++))
    done

    echo "$port"
}

load_vm(){
    local file="$1/vm.conf"

    [[ -f "$file" ]] || die "Missing vm.conf in $1"

    local -A allowed=(
        [NAME]=1
        [ISO]=1
        [DRIVER_ISO]=1
        [DISK]=1
        [OVMF_CODE]=1
        [OVMF_VARS]=1
        [RAM_MB]=1
        [VCPUS]=1
        [CREATED]=1
    )

    while IFS='=' read -r key val; do
        [[ -n "${allowed[$key]+x}" ]] || continue

        val="${val%$'\r'}"

        printf -v "$key" '%s' "$val"
    done < "$file"
}

start_tpm(){
    local name="$1"
    local vmdir
    vmdir=$(vm_dir "$name")

    require swtpm

    local socket
    socket=$(vm_tpm_socket "$name")

    local state_dir="$vmdir/tpm"

    mkdir -p "$state_dir"

    rm -f "$socket"

    swtpm socket \
        --tpm2 \
        --tpmstate "dir=$state_dir" \
        --ctrl "type=unixio,path=$socket" \
        --daemon

    local tpm_pid=""

    # swtpm --daemon does not consistently expose a PID in all versions,
    # so identify the process by its unique control socket.
    for _ in {1..20}; do
        [[ -S "$socket" ]] && break
        sleep 0.1
    done

    [[ -S "$socket" ]] ||
        die "Failed to start swtpm"

    tpm_pid=$(pgrep -f "swtpm.*path=$socket" | head -n1 || true)

    if [[ -n "$tpm_pid" ]]; then
        echo "$tpm_pid" > "$(vm_tpm_pid_file "$name")"
    fi
}

stop_tpm(){
    local name="$1"
    local socket
    socket=$(vm_tpm_socket "$name")

    if [[ -S "$socket" ]]; then
        swtpm_ioctl \
            --unix "$socket" \
            --save \
            &>/dev/null || true
    fi

    local pidfile
    pidfile=$(vm_tpm_pid_file "$name")

    if [[ -f "$pidfile" ]]; then
        local pid
        pid=$(cat "$pidfile" 2>/dev/null || true)

        if [[ "$pid" =~ ^[0-9]+$ ]]; then
            kill "$pid" 2>/dev/null || true
        fi

        rm -f "$pidfile"
    fi

    rm -f "$socket"
}

cmd_create(){
    local name="${1:-}"
    local iso="${2:-}"
    local driver_iso="${3:-}"

    [[ -n "$name" && -n "$iso" ]] ||
        die "Usage: qvm create <name> <windows.iso> [virtio.iso]"

    [[ "$name" =~ ^[a-zA-Z0-9][a-zA-Z0-9_-]*$ ]] ||
        die "Invalid VM name '$name'."

    [[ -f "$iso" ]] ||
        die "Windows ISO not found: $iso"

    if [[ -n "$driver_iso" && ! -f "$driver_iso" ]]; then
        die "VirtIO ISO not found: $driver_iso"
    fi

    require qemu-system-x86_64
    require qemu-img
    find_ovmf

    local vmdir="$VM_DIR/$name"

    [[ -d "$vmdir" ]] &&
        die "VM '$name' already exists"

    mkdir -p "$vmdir"

    local disk="$vmdir/disk.qcow2"

    qemu-img create \
        -f qcow2 \
        "$disk" \
        "$DISK_SIZE_DEFAULT" \
        -q

    cp "$OVMF_VARS" "$vmdir/ovmf-vars.fd"

    cat > "$vmdir/vm.conf" <<EOF
NAME=$name
ISO=$iso
DRIVER_ISO=$driver_iso
DISK=$disk
OVMF_CODE=$OVMF_CODE
OVMF_VARS=$vmdir/ovmf-vars.fd
RAM_MB=$RAM_MB_DEFAULT
VCPUS=$VCPUS_DEFAULT
CREATED=$(date -Iseconds)
EOF

    ok "Created VM: $name"
    info "RAM: ${RAM_MB_DEFAULT} MB"
    info "vCPUs: $VCPUS_DEFAULT"
    info "Disk: $DISK_SIZE_DEFAULT"

    if [[ "$SECURE_BOOT" == 1 ]]; then
        ok "Secure Boot-capable OVMF detected"
    else
        warn "Secure Boot OVMF not detected"
    fi

    [[ -n "$driver_iso" ]] &&
        info "VirtIO drivers: $driver_iso"
}

cmd_launch(){
    local name="${1:-}"
    local flag="${2:-}"

    [[ -n "$name" ]] ||
        die "Usage: qvm launch <name> [--no-iso]"

    [[ -z "$flag" || "$flag" == "--no-iso" ]] ||
        die "Unknown option: '$flag'. Valid: --no-iso"

    local vmdir
    vmdir=$(vm_dir "$name")

    [[ -d "$vmdir" ]] ||
        die "VM '$name' not found. Run: qvm list"

    load_vm "$vmdir"

    local lockfile="$vmdir/.launch.lock"

    exec 9>"$lockfile"

    flock -n 9 ||
        die "VM '$name' is already launching."

    cleanup_stale_files "$name"

    if vm_is_running "$name"; then
        local port
        port=$(cat "$(vm_port_file "$name")" 2>/dev/null || echo "?")

        warn "Already running → spice://localhost:$port"
        return
    fi

    require qemu-system-x86_64
    require ss

    find_ovmf

    local port
    port=$(next_spice_port)

    echo "$port" > "$(vm_port_file "$name")"

    local log
    log=$(vm_log_file "$name")

    start_tpm "$name"

    local tpm_socket
    tpm_socket=$(vm_tpm_socket "$name")
    
    local cmd=(
        qemu-system-x86_64
    
        # Machine / acceleration
        -machine "q35,accel=kvm"
        -cpu host
        -smp "$VCPUS"
        -m "$RAM_MB"
    
        # UEFI
        -drive "if=pflash,format=raw,readonly=on,file=$OVMF_CODE"
        -drive "if=pflash,format=raw,file=$OVMF_VARS"
    
        # TPM 2.0
        -chardev "socket,id=chrtpm,path=$tpm_socket"
        -tpmdev "emulator,id=tpm0,chardev=chrtpm"
        -device "tpm-crb,tpmdev=tpm0"
    
        # Graphics
        -device "virtio-gpu-pci"
        -spice "port=$port,disable-ticketing=on"
    
        # SPICE agent
        -device "virtio-serial-pci"
        -chardev "spicevmc,id=vdagent,name=vdagent"
        -device "virtserialport,chardev=vdagent,name=com.redhat.spice.0"
    
        # Network
        -nic "user,model=virtio-net-pci"
    
        # System disk
        -drive "file=$DISK,if=virtio,format=qcow2,cache=none"
    
        # USB tablet
        -device "usb-ehci"
        -device "usb-tablet"
    
        # No local QEMU window
        -display none
    
        # Boot order
        -boot "order=dc"
    )

    # Windows installation ISO
    if [[ "$flag" != "--no-iso" && -f "$ISO" ]]; then
        cmd+=(
            -drive "file=$ISO,media=cdrom,readonly=on"
        )

        info "Windows ISO attached"
    elif [[ "$flag" != "--no-iso" && ! -f "$ISO" ]]; then
        warn "ISO not found at $ISO"
        info "Booting from disk"
    else
        info "Booting from disk"
    fi

    # VirtIO driver ISO
    if [[ -n "${DRIVER_ISO:-}" && -f "$DRIVER_ISO" ]]; then
        cmd+=(
            -drive "file=$DRIVER_ISO,media=cdrom,readonly=on"
        )

        info "VirtIO driver ISO attached"
    fi

    info "Starting TPM 2.0..."

    "${cmd[@]}" >"$log" 2>&1 &

    local pid=$!

    echo "$pid" > "$(vm_pid_file "$name")"

    sleep 1

    if ! vm_is_running "$name"; then
        stop_tpm "$name"
        rm -f "$(vm_pid_file "$name")"

        die "QEMU exited immediately. Check: $log"
    fi

    ok "Running → PID $pid | spice://localhost:$port"

    if command -v remote-viewer &>/dev/null; then
        remote-viewer "spice://localhost:$port" &
    elif command -v virt-viewer &>/dev/null; then
        virt-viewer --connect "spice://localhost:$port" &
    else
        warn "No SPICE viewer found."
        info "Install: virt-viewer"
        info "Connect to: spice://localhost:$port"
    fi
}

cmd_stop(){
    local name="${1:-}"

    [[ -n "$name" ]] ||
        die "Usage: qvm stop <name>"

    if ! vm_is_running "$name"; then
        stop_tpm "$name"
        rm -f "$(vm_pid_file "$name")"

        die "VM '$name' is not running"
    fi

    local pid
    pid=$(cat "$(vm_pid_file "$name")")

    info "Stopping $name..."

    kill -SIGTERM "$pid" 2>/dev/null || true

    for _ in {1..10}; do
        sleep 1

        if ! vm_is_running "$name"; then
            break
        fi
    done

    if vm_is_running "$name"; then
        warn "SIGTERM ignored, sending SIGKILL..."

        kill -SIGKILL "$pid" 2>/dev/null || true

        sleep 1
    fi

    if vm_is_running "$name"; then
        die "Failed to stop VM '$name'"
    fi

    stop_tpm "$name"

    rm -f "$(vm_pid_file "$name")"

    ok "Stopped $name"
}

cmd_list(){
    sep

    [[ -d "$VM_DIR" ]] || {
        warn "No VMs found."
        sep
        return
    }

    local found=false

    for d in "$VM_DIR"/*/; do
        [[ -f "$d/vm.conf" ]] || continue

        found=true

        load_vm "$d"

        if vm_is_running "$NAME"; then
            local port
            port=$(cat "$d/spice.port" 2>/dev/null || echo "?")

            echo -e \
                "  ${B}$NAME${Z}  ${G}running${Z}  spice://localhost:$port"
        else
            echo -e \
                "  ${B}$NAME${Z}  ${Y}stopped${Z}"
        fi
    done

    [[ "$found" == false ]] &&
        warn "No VMs found."

    sep
}

cmd_disable(){
    local name="${1:-}"

    [[ -n "$name" ]] ||
        die "Usage: qvm disable <name>"

    local vmdir
    vmdir=$(vm_dir "$name")

    [[ -d "$vmdir" ]] ||
        die "VM '$name' not found"

    if vm_is_running "$name"; then
        warn "Stopping running VM..."
        cmd_stop "$name"
    fi

    read -rp "Type '$name' to confirm deletion: " confirm

    [[ "$confirm" == "$name" ]] ||
        die "Aborted"

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

    echo -e "  ${C}qvm create  <name> <windows.iso> [virtio.iso]${Z}"
    echo -e "                              create a new VM"

    echo -e "  ${C}qvm launch  <name>${Z}      boot with installation ISO"

    echo -e "  ${C}qvm launch  <name> --no-iso${Z}"
    echo -e "                              boot from disk"

    echo -e "  ${C}qvm stop    <name>${Z}      gracefully stop VM"

    echo -e "  ${C}qvm list${Z}               show VMs and status"

    echo -e "  ${C}qvm disable <name>${Z}      stop and delete VM"

    echo -e "  ${C}qvm version${Z}             print version"

    sep

    echo -e "  Environment overrides:"

    echo -e "    QVM_DIR   VM storage directory  (default: ~/vms)"
    echo -e "    QVM_RAM   RAM in MB             (default: 8192)"
    echo -e "    QVM_CPUS  vCPU count            (default: 4)"
    echo -e "    QVM_DISK  Disk size             (default: 120G)"

    sep
}

case "${1:-}" in
    create)
        cmd_create "${2:-}" "${3:-}" "${4:-}"
        ;;

    launch)
        cmd_launch "${2:-}" "${3:-}"
        ;;

    stop)
        cmd_stop "${2:-}"
        ;;

    list)
        cmd_list
        ;;

    disable)
        cmd_disable "${2:-}"
        ;;

    version)
        cmd_version
        ;;

    help|--help|-h)
        cmd_help
        ;;

    *)
        cmd_help
        ;;
esac

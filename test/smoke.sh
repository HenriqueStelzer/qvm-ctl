m#!/usr/bin/env bash
# Smoke tests for qvm-ctl
# Runs without KVM/QEMU runtime — tests CLI logic only

set -uo pipefail

QVM="./qvm-ctl.sh"
export QVM_DIR
QVM_DIR=$(mktemp -d)

trap 'rm -rf "$QVM_DIR"' EXIT

PASS=0 FAIL=0

check(){
    local desc="$1"; shift
    if "$@" >/dev/null 2>&1; then
        echo "  ✓ $desc"m
        ((PASS++))
    else
        echo "  ✗ $desc"
        ((FAIL++))
    fi
}

check_fail(){
    local desc="$1"; shift
    if ! "$@" >/dev/null 2>&1; then
        echo "  ✓ $desc"
        ((PASS++))
    else
        echo "  ✗ $desc (expected failure)"
        ((FAIL++))
    fi
}

check_output(){
    local desc="$1" pattern="$2"; shift 2
    if "$@" 2>&1 | grep -q "$pattern"; then
        echo "  ✓ $desc"
        ((PASS++))
    else
        echo "  ✗ $desc"
        ((FAIL++))
    fi
}

echo "=== qvm-ctl smoke tests ==="
echo ""

# --- help / version ---
echo "Commands:"
check "help exits 0"              bash "$QVM" help
check "--help exits 0"            bash "$QVM" --help
check "-h exits 0"                bash "$QVM" -h
check "version exits 0"           bash "$QVM" version
check_output "version shows 1.0.0" "1.0.0" bash "$QVM" version

# --- create ---
echo ""
echo "Create:"

FAKE_ISO="$QVM_DIR/fake.iso"
touch "$FAKE_ISO"

check      "create valid VM"                bash "$QVM" create testvm "$FAKE_ISO"
check      "vm.conf exists"                 test -f "$QVM_DIR/testvm/vm.conf"
check      "disk.qcow2 exists"             test -f "$QVM_DIR/testvm/disk.qcow2"
check_fail "create duplicate VM"            bash "$QVM" create testvm "$FAKE_ISO"
check_fail "create with no args"            bash "$QVM" create
check_fail "create with missing ISO"        bash "$QVM" create badvm /nonexistent.iso
check_fail "create with invalid name (-x)"  bash "$QVM" create -x "$FAKE_ISO"

# --- list ---
echo ""
echo "List:"
check_output "list shows testvm"   "testvm"  bash "$QVM" list
check_output "list shows stopped"  "stopped" bash "$QVM" list

# --- launch validation ---
echo ""
echo "Launch validation:"
check_fail "launch nonexistent VM"          bash "$QVM" launch noexist
check_fail "launch with bad flag"           bash "$QVM" launch testvm --garbage

# --- disable ---
echo ""
echo "Disable:"
check_fail "disable nonexistent VM"         bash "$QVM" disable noexist

# disable with piped confirmation
echo "testvm" | bash "$QVM" disable testvm >/dev/null 2>&1
if [[ ! -d "$QVM_DIR/testvm" ]]; then
    echo "  ✓ disable with confirm"
    ((PASS++))
else
    echo "  ✗ disable with confirm"
    ((FAIL++))
fi

# --- empty state ---
echo ""
echo "Empty state:"
rm -rf "$QVM_DIR"
mkdir -p "$QVM_DIR"
check "list on empty dir"          bash "$QVM" list

# --- summary ---
echo ""
echo "────────────────────────────"
echo "  $PASS passed, $FAIL failed"
echo "────────────────────────────"

[[ $FAIL -eq 0 ]] || exit 1

#!/usr/bin/env nu
# Run all quality checks

def main [] {
    print "🔍 Running all quality checks...\n"

    let checks = [
        {name: "Format Check", command: "nu scripts/format.nu --check"},
        {name: "Lint", command: "nu scripts/lint.nu"},
        {name: "Tests", command: "nu scripts/test.nu"},
        {name: "Audit", command: "nu scripts/audit.nu"}
    ]

    mut failed = []

    for check in $checks {
        print $"Running ($check.name)..."

        let result = (do -i { nu -c $check.command } | complete)

        if $result.exit_code != 0 {
            $failed = ($failed | append $check.name)
            print $"❌ ($check.name) failed\n"
        } else {
            print $"✅ ($check.name) passed\n"
        }
    }

    if ($failed | is-empty) {
        print "✅ All checks passed!"
    } else {
        print $"❌ Failed checks: ($failed | str join ', ')"
        exit 1
    }
}

# Entry point
main

#!/usr/bin/env nu
# Run CI checks locally (simulates CI environment)

def main [] {
    print "🤖 Running CI checks locally...\n"

    let steps = [
        {name: "Setup", command: "nu scripts/setup.nu"},
        {name: "Format Check", command: "nu scripts/format.nu --check"},
        {name: "Lint", command: "nu scripts/lint.nu"},
        {name: "Build", command: "nu scripts/build.nu"},
        {name: "Test", command: "nu scripts/test.nu"},
        {name: "Audit", command: "nu scripts/audit.nu"}
    ]

    mut failed = []
    mut step_num = 1

    for step in $steps {
        print $"Step ($step_num)/($steps | length): ($step.name)"

        let result = (do -i { nu -c $step.command } | complete)

        if $result.exit_code != 0 {
            $failed = ($failed | append $step.name)
            print $"❌ ($step.name) failed\n"
        } else {
            print $"✅ ($step.name) passed\n"
        }

        $step_num = $step_num + 1
    }

    if ($failed | is-empty) {
        print "✅ All CI checks passed!"
        print "   Your changes are ready for CI/CD pipeline"
    } else {
        print $"❌ CI checks failed: ($failed | str join ', ')"
        print "   Fix these issues before pushing"
        exit 1
    }
}

# Entry point
main

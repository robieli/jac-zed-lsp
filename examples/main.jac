# Welcome to Jac in Zed!
# This sample file tests syntax highlighting, code outline, bracket matching, and the Jac LSP.

"""A module docstring describing this Jac example."""

enum Language {
    JAC = "Jac",
    PYTHON = "Python"
}

node Item {
    has name: str;
    has value: int = 42;
}

walker Greeter {
    has greeting: str = "Hello from Zed Jac Extension!";

    can greet with entry {
        print(self.greeting);
    }
}

walker ItemVisitor {
    has target_name: str;

    can visit_item with Item entry {
        print(f"Visiting item '{here.name}' with value {here.value}");
    }
}

def calculate_sum(a: int, b: int) -> int {
    return a + b;
}

# Testing inline Python injection
def inline_python_demo {
    ::py::
    import math
    result = math.sqrt(16)
    print(f"Inline Python calculated sqrt(16) = {result}")
    ::py::
}

with entry {
    print("Executing Jac main entry point...");

    # Instantiate and run walker
    root spawn Greeter();

    # Function call
    total = calculate_sum(10, 32);
    print(f"Calculated sum: {total}");

    # Run inline python block
    inline_python_demo();
}

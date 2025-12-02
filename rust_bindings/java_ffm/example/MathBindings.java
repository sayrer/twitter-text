import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;

public class MathBindings {
    static MethodHandle addNumbers;
    static MethodHandle multiplyNumbers;

    static {
        // Find the library in the runfiles directory
        // Bazel names the shared library libmath.so on all platforms
        Path libPath = Path.of("rust_bindings/java_ffm/example/libmath.so");

        Linker linker = Linker.nativeLinker();
        SymbolLookup lib = SymbolLookup.libraryLookup(libPath, Arena.global());

        addNumbers = linker.downcallHandle(
            lib.find("add_numbers").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_INT,
                ValueLayout.JAVA_INT,
                ValueLayout.JAVA_INT
            )
        );

        multiplyNumbers = linker.downcallHandle(
            lib.find("multiply_numbers").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_INT,
                ValueLayout.JAVA_INT,
                ValueLayout.JAVA_INT
            )
        );
    }
}

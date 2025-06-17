package io.github.ran.zip4j_abi;

//import com.oracle.svm.core.c.libc.TemporaryBuildDirectoryProvider;
//import com.oracle.svm.core.util.VMError;
//import com.oracle.svm.hosted.FeatureImpl;
//import com.oracle.svm.hosted.NativeImageGenerator;
//import org.graalvm.nativeimage.ImageSingletons;
import org.graalvm.nativeimage.hosted.Feature;
//
//import java.io.IOException;
//import java.nio.file.*;
//import java.nio.file.attribute.BasicFileAttributes;

public class StaticLibrary implements Feature {
    /**
     * Copy the native library files to the output directory.
     * <p>
     * Code modified from <a href="https://github.com/oracle/graal/issues/3053">...</a>
     */
//    @Override
//    public void afterImageWrite(AfterImageWriteAccess afterImageWriteAccess) {
//        if (afterImageWriteAccess instanceof FeatureImpl.AfterImageWriteAccessImpl access) {
//            try {
//                Path outputDirectory = NativeImageGenerator.generatedFiles(access.getUniverse().getBigBang().getOptions());
//                if (Files.notExists(outputDirectory)) Files.createDirectory(outputDirectory);
//                Files.walkFileTree(ImageSingletons.lookup(TemporaryBuildDirectoryProvider.class).getTemporaryBuildDirectory(), new SimpleFileVisitor<>() {
//                    @Override
//                    public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
//                        String fileName = file.getFileName().toString();
//                        System.out.println(fileName);
//                        if (fileName.endsWith(".o") || fileName.endsWith(".h") || fileName.endsWith(".a") || fileName.endsWith(".lib") || fileName.endsWith(".obj")) {
//                            Files.copy(file, outputDirectory.resolve(fileName).toAbsolutePath(), StandardCopyOption.REPLACE_EXISTING);
//                        }
//                        return FileVisitResult.CONTINUE;
//                    }
//                });
//            } catch (IOException e) {
//                VMError.shouldNotReachHere("Failed to copy libraries from temporary build directory", e);
//            }
//        }
//    }

    // Files in temp directory:
//    AArch64LibCHelperDirectives.c
//    AArch64LibCHelperDirectives.exe
//    AArch64LibCHelperDirectives.obj
//    AMD64LibCHelperDirectives.c
//    AMD64LibCHelperDirectives.exe
//    AMD64LibCHelperDirectives.obj
//    BuiltinDirectives.c
//    BuiltinDirectives.exe
//    BuiltinDirectives.obj
//    detect-cl-version-info.c
//    JNIHeaderDirectives.c
//    JNIHeaderDirectives.exe
//    JNIHeaderDirectives.obj
//    JNIHeaderDirectivesJDKLatest.c
//    JNIHeaderDirectivesJDKLatest.exe
//    JNIHeaderDirectivesJDKLatest.obj
//    RISCV64LibCHelperDirectives.c
//    RISCV64LibCHelperDirectives.exe
//    RISCV64LibCHelperDirectives.obj
//    WindowsDirectives.c
//    WindowsDirectives.exe
//    WindowsDirectives.obj
//    <your library>.exp
//    <your library>.lib
//    <your library>.obj
}

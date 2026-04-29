import java.io.IOException;

public class VulnCommand {
      public void runCommand(String cmd) throws IOException {
                // VULNERABILITY: OS Command Injection via Runtime.exec with user input
          Runtime.getRuntime().exec(cmd);
      }

    public static void main(String[] args) throws IOException {
              VulnCommand vc = new VulnCommand();
              vc.runCommand(args[0]);
    }
}

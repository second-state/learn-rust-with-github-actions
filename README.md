# Working with VSCode Codespaces

<p>
    <a href="https://online.visualstudio.com/environments/new?name=Learn%20Rust&repo=second-state/learn-rust-with-github-actions">
        <img src="https://img.shields.io/endpoint?style=social&url=https%3A%2F%2Faka.ms%2Fvso-badge">
    </a>
</p>

VSCode is the [most popular IDE](https://blog.rust-lang.org/2020/04/17/Rust-survey-2019.html#ides-and-tooling---a-closer-look) for developing Rust applications. This template works with the VSCode Codespaces online IDE! Just [fork it](https://github.com/second-state/learn-rust-with-github-actions/fork), 
open it in the online IDE, write Rust code, build, and run it directly from the IDE. No software download or install needed! 

> VSCode Codespaces runs entirely in your browser and costs around $1 per work day. It is cheaper than a cup of coffee in the office. Alternatively, in steps 1-2 below, you could use locally installed VSCode and Docker, and [launch the IDE with your remote git repository](https://code.visualstudio.com/remote-tutorials/containers/getting-started).

First, open the [VSCode Codespaces](https://online.visualstudio.com/) web site and login with your Azure account. You can get a [free Azure account](https://azure.microsoft.com/en-us/free/).

Next, create a new Codespace. Put your forked repository into the Git Repository field.

![Create a new Codespace](https://www.secondstate.io/external/img/vscode_create.png)

Then open the `main.rs` and `Cargo.toml` files and code your fancy Rust application!

![Code in Codespace](https://www.secondstate.io/external/img/vscode_code.png)

Click on the Run Debug button in the left panel, and then the Run button at the top. 

![Run the Rust program](https://www.secondstate.io/external/img/vscode_run_button.png)

See your Rust application run. Its standard output is displayed in the Terminal window in the IDE.

![See output in terminal](https://www.secondstate.io/external/img/vscode_run.png)

From the Terminal window, you can run any Linux command including `cargo` and `rustup`. For example, you can manually run the executable binary in the `target` directory.

![Access the file system in the terminal](https://www.secondstate.io/external/img/vscode_terminal.png)

Now, go [learn Rust](https://doc.rust-lang.org/book/) or [try out the examples](https://doc.rust-lang.org/stable/rust-by-example/) in your new IDE!







<h1 align="center">
  <img
    src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png"
    height="30"
    width="0px"
  />
    🌋 vCLI
  <img
    src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png"
    height="30"
    width="0px"
  />
</h1>

## 💭 About

vCLI is my tiny CLI for init C++ (maybe support multiple language) Project, which is written in Rust🦀.

## 📦 Install

You can easily install vCLI using `cargo`.
``` sh
cargo install vcli
```

## 🚀 Usage

You can use vCLI just wiht your target project name

``` sh
vcli <project-name>
```

**vCLI use cpp as default template**, so you can use it to init cpp project without additional flag.
simple example as follow: 
``` sh
vcli hello-world
```

In Addition, you need add language flag `-l` or `--language` to specify the lang of target, if you want use it for extra template. Simple shell example as follow: 
```sh
vcli hello-world -l shell
```

### ✨ Supported Language
- C++
- Shell

### 🌲 Template Structure (CPP version)
```
├── app/
├── include
│   ├── utils/
│   ├── hello.h
│   └── your header file...
├── scripts/
│   ├── run.sh
│   └── your scripts file..
├── src
│   ├── core/ 
│   ├── utils/
│   ├── CMakeLists.txt
│   ├── hello.cc
│   └── your source file...
├── tests/
├── CMakeLists.txt
└── main.cc
```

## ✅ TODO
- [ ] Add more templates
- [ ] Support for customised templates
- [ ] More and more

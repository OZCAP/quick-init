## QUICK INIT
<img src="https://github.com/OZCAP/quick-init/blob/main/logo.svg" alt="Quick Init Logo" width="100" height="100">
CLI Tool to quickly start up React projects with Tailwind automatically configured.

Typescript is selected **as default** If JS is to be used, it must be specified with an option parameter.

Compatible templates: **Vite, Next JS**

### Installation
#### Mac OSX (Brew):
```bash
brew tap ozcap/quick-init && brew install quick-init
```

### Documentation
#### Usage:
    quick-init <PROJECT_NAME> [OPTIONS]

#### Arguments:
    <PROJECT_NAME>    Name of the project to be initialised

#### Options:
    -h, --help                   Print help information
    -j, --javascript             Use Javascript instead of Typescript
    -t, --template <TEMPLATE>    [vite|next] [default: vite]
    -V, --version                Print version information
    
### TODO
- [x] Configure tailwind configuration automatically
- [x] Add choice of NextJs as well as Vite react
- [ ] Allow local config to be loaded/set with chosen dependancies.

## QUICK INIT
CLI Tool to quickly start up React projects with Tailwind readily installed and configured,
Typescript is selected **as default**. If JS is to be used, it must be specified with an option.

Compatible templates: **Vite, Next JS**

#### USAGE:
    quick-init <NAME> [OPTIONS]

#### ARGS:
    <NAME>    Name of the project to be initialised

#### OPTIONS:
    -h, --help                   Print help information
    -j, --javascript             Use Javascript instead of Typescript
    -t, --template <TEMPLATE>    [vite|next] [default: vite]
    -V, --version                Print version information
    
### TODO
- [x] Configure tailwind configuration automatically
- [x] Add choice of NextJs as well as Vite react
- [ ] Allow local config to be loaded/set with chosen dependancies.

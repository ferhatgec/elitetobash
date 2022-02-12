# [elite](https://github.com/ferhatgec/elite)tobash
## [elite](https://github.com/ferhatgec/elite) -> bash converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetobash"
set HOME        as env "HOME"


for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output
```shell
#!/usr/bin/env bash
arch=$(uname -i)
arch="${arch,,}"
if [[ "0.1" != "0.1" ]]; then
 printf "elite: Required higher version\n"
 exit
fi
ProjectName="elitetobash"
HOME="/home/gech"
if [[ $# -ge 1 && $1 == "install" ]];
then
 cargo install --path .
 if [[ -f "/home/gech/.cargo/bin/elitetobash" ]];
then
  printf 'elitetobash installed to /home/gech/.cargo/bin/elitetobash.\n'
fi
 exit
fi

```

### elitetobash licensed under the terms of MIT License

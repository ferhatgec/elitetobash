// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_bash {
    use elite::ast::EliteKeywords;

    fn replace(data: String) -> String {
        data.replace('\'', "\\'").replace("\0", "")
    }

    pub fn parse(data: elite::parser::EliteParser) -> String {
        let mut regenerated_code = String::from("\
        #!/usr/bin/env bash\n\
        arch=$(uname -i)
        arch=\"${arch,,}\"\n");
        let mut line = 0u32;
        let mut is_for = false;

        for x in data.ast_nodes.data {
            match x.__type {
                EliteKeywords::Set => {
                    regenerated_code.push_str(
                        format!("{}{}=\"{}\"\n", " ".repeat(line as usize), x.__name, replace(x.__data)).as_str());
                }
                EliteKeywords::Print => {
                    regenerated_code.push_str(
                        format!("{}printf \"{}\"\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Println => {
                    regenerated_code.push_str(
                        format!("{}printf '{}\\n'\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Use => {}
                EliteKeywords::RequiredVersion => {
                    regenerated_code.push_str(format!("if [[ \"{}\" != \"{}\" ]]; then\n{}",
                                                            replace(x.__name),
                                                            replace(x.__data),
                                                            " printf \"elite: Required higher version\\n\"\n exit\nfi\n").as_str());
                }
                EliteKeywords::Change => {}
                EliteKeywords::IfArg => {
                    regenerated_code.push_str(format!("{}if [[ \"{}\"", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::LeftParenthese => {}
                EliteKeywords::RightParenthese => {}
                EliteKeywords::LeftSqBracket => {
                    if is_for { is_for = false; continue; }
                    regenerated_code.push_str("then\n"); line += 1;
                }
                EliteKeywords::RightSqBracket => {
                    if line < 1 { continue }
                    regenerated_code.push_str("fi\n"); line -= 1;
                }
                EliteKeywords::Eq => {
                    regenerated_code.push_str(format!(" == \"{}\" ]];\n", replace(x.__data)).as_str());
                }
                EliteKeywords::UnEq => {
                    regenerated_code.push_str(format!(" != \"{}\" ]];\n", replace(x.__data)).as_str());
                }
                EliteKeywords::Signal => {
                    if x.__name == "exit" {
                        regenerated_code.push_str(format!("{}exit\n", " ".repeat(line as usize)).as_str());
                    } else if x.__name == "start" {
                        is_for = true;
                    }
                }
                EliteKeywords::Exec => {
                    regenerated_code.push_str(format!("{}{}\n", " ".repeat(line as usize), replace(x.__name)).as_str());
                }
                EliteKeywords::AddSource => {}
                EliteKeywords::Append => {}
                EliteKeywords::Exit => {
                    regenerated_code.push_str(format!("{}exit\n", " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Specific => {
                    match x.__data.as_str() {
                        "x86" => regenerated_code.push_str(
                            format!("{}if [[ $arch == \"x86_32\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "amd64" => regenerated_code.push_str(
                                format!("{}if [[ $arch == \"amd64\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "windows" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"cywin\"' ]];\n", " ".repeat(line as usize)).as_str()),
                        "macos" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"darwin\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "linux" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"linux-gnu\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "freebsd" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"freebsd\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "netbsd" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"netbsd\" ]];\n", " ".repeat(line as usize)).as_str()),
                        "android" => regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"android\" ]];\n", " ".repeat(line as usize)).as_str()),
                        _ =>
                            // other platforms are not directly supported but this is may be TODO.
                            regenerated_code.push_str(
                            format!("{}if [[ $OSTYPE == \"{}\" ]];\n", " ".repeat(line as usize), replace(x.__data)).as_str())

                    }

                }
                EliteKeywords::Argument => {
                    regenerated_code.push_str(
                        format!("{}if [[ $# -ge 1 && $1 == \"{}\" ]];\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Exists => {
                    regenerated_code.push_str(
                        format!("{}if [[ -f \"{}\" ]];\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Undefined => {},
                _ => {}
            }
        }

        regenerated_code
    }
}
{{ if bin_path }}
proto.exe run {bin} --bin "{bin_path}" -- {before_args} %* {after_args}
{{ else }}
proto.exe run {bin} -- {before_args} %* {after_args}
{{ endif }}

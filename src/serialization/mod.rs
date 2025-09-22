use crate::core::JsonValue;
use crate::parser::StringEscaper;

impl JsonValue{

    pub(crate) fn to_json_string(&self) -> String {
        self.format(0, false)
    }
    pub(crate)  fn to_json_string_pretty(&self) -> String{
        self.format(0, true)
    }
    fn format(&self, indent: usize, pretty: bool) -> String{
        use JsonValue::*;
        match self {
            Null => "null".to_string(),
            Bool(b) => b.to_string(),
            Number(n) => n.to_string(),
            String(s) =>  format!("\"{}\"", StringEscaper::escape(s)),
            Array(arr) => self.format_array(arr, indent, pretty),
            Object(obj) => self.format_object(obj, indent, pretty),
        }
    }
    fn format_array(&self, arr: &[JsonValue], indent: usize, pretty: bool) -> String {
        if arr.is_empty() {
            return "[]".to_string();
        }

        let items: Vec<String> = arr
            .iter()
            .map(|v| {
                if pretty {
                    format!("{}{}", indent_str(indent + 1), v.format(indent + 1, true))
                } else {
                    v.format(indent + 1, false)
                }
            })
            .collect();

        if pretty {
            format!("[\n{}\n{}]", items.join(",\n"), indent_str(indent))
        } else {
            format!("[{}]", items.join(","))
        }
    }

    fn format_object(&self, obj: &[(String, JsonValue)], indent: usize, pretty: bool) -> String {
        if obj.is_empty() {
            return "{}".to_string();
        }

        let items: Vec<String> = obj
            .iter()
            .map(|(k, v)| {
                if pretty {
                    format!(
                        "{}\"{}\": {}",
                        indent_str(indent + 1),
                        StringEscaper::escape(k),
                        v.format(indent + 1, true)
                    )
                } else {
                    format!("\"{}\":{}", StringEscaper::escape(k), v.format(indent + 1, false))
                }
            })
            .collect();

        if pretty {
            format!("{{\n{}\n{}}}", items.join(",\n"), indent_str(indent))
        } else {
            format!("{{{}}}", items.join(","))
        }
    }
}

fn indent_str(level: usize) -> String {
    "    ".repeat(level)
}
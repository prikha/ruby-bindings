use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "# This file is autogenerated by {{ helper generated-by }}

module LibRubyParser
  # Module with all known Node sub-types
  module Nodes
{{ each node }}<dnl>
{{ helper node-comment }}<dnl>
    class {{ helper node-camelcase-name }} < Node
{{ each node-field }}<dnl>
{{ helper node-field-comment }}<dnl>
      attr_reader :{{ helper node-field-name }}

{{ end }}<dnl>
    end

{{ end }}<dnl>
  end
end
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("../lib/lib-ruby-parser/nodes.rb", contents).unwrap();
}

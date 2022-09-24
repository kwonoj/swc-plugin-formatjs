import { test } from "@jest/globals";
import * as path from "path";
import {
  ExtractedMessageDescriptor,
  transform,
  transformAndCheck,
} from "./transform";

const getFixturePath = (fixtureName: string) => path.resolve(__dirname, 'fixtures', fixtureName);

test("additionalComponentNames", function () {
  expect(
    transformAndCheck("additionalComponentNames", {
      additionalComponentNames: ["CustomMessage"],
    })
  ).toMatchInlineSnapshot(`
    {
      "code": "import React, { Component } from 'react';
    function CustomMessage() {}
    export default class Foo extends Component {
        render() {
            return /*#__PURE__*/ React.createElement(CustomMessage, {
                id: "greeting-world",
                defaultMessage: "Hello World!"
            });
        }
    }",
      "data": {
        "messages": [
          {
            "defaultMessage": "Hello World!",
            "description": "Greeting to the world",
            "id": "greeting-world",
          },
        ],
        "meta": {},
      },
    }
  `);
});

test.skip("additionalFunctionNames", function () {
  transformAndCheck("additionalFunctionNames", {
    additionalFunctionNames: ["t"],
  });
});

test.skip("ast", function () {
  transformAndCheck("ast", {
    ast: true,
  });
});

test.skip("defineMessage", function () {
  transformAndCheck("defineMessage");
});

test.skip("descriptionsAsObjects", function () {
  transformAndCheck("descriptionsAsObjects");
});

test.skip("defineMessages", function () {
  transformAndCheck("defineMessages");
});
test("empty", function () {
  expect(transformAndCheck("empty")).toMatchInlineSnapshot(`
    {
      "code": "import React, { Component } from 'react';
    import { defineMessage } from 'react-intl';
    export default class Foo extends Component {
        render() {
            return /*#__PURE__*/ React.createElement("div", null);
        }
    }",
      "data": {
        "messages": [],
        "meta": {},
      },
    }
  `);
});
test.skip("extractFromFormatMessageCall", function () {
  transformAndCheck("extractFromFormatMessageCall");
});
test.skip("extractFromFormatMessageCallStateless", function () {
  transformAndCheck("extractFromFormatMessageCallStateless");
});
test.skip("formatMessageCall", function () {
  transformAndCheck("formatMessageCall");
});
test("FormattedMessage", function () {
  expect(transformAndCheck("FormattedMessage")).toMatchInlineSnapshot(`
    {
      "code": "import React, { Component } from 'react';
    import { FormattedMessage } from 'react-intl';
    export default class Foo extends Component {
        render() {
            return /*#__PURE__*/ React.createElement(FormattedMessage, {
                id: "foo.bar.baz",
                defaultMessage: "Hello World!"
            });
        }
    }",
      "data": {
        "messages": [
          {
            "defaultMessage": "Hello World!",
            "description": "The default message.",
            "id": "foo.bar.baz",
          },
        ],
        "meta": {},
      },
    }
  `);
});
test.skip("inline", function () {
  transformAndCheck("inline");
});
test.skip("templateLiteral", function () {
  expect(transformAndCheck("templateLiteral")).toMatchInlineSnapshot(`
    {
      "code": "import React, { Component } from 'react';
    import { FormattedMessage, defineMessage } from 'react-intl';
    defineMessage({
        id: \`template\`,
        defaultMessage: \`should remove newline and extra spaces\`
    });
    export default class Foo extends Component {
        render() {
            return /*#__PURE__*/ React.createElement(FormattedMessage, {
                id: "foo.bar.baz",
                defaultMessage: \`Hello World!\`
            });
        }
    }",
      "data": {
        "messages": [{
          "defaultMessage": "should remove newline and extra spaces",
          "description": undefined,
          "id": "template",
        }, {
          "defaultMessage": "Hello World!",
          "description": "The default message.",
          "id": "foo.bar.baz",
        }],
        "meta": {},
      },
    }
  `);
});

test.skip("idInterpolationPattern", function () {
  transformAndCheck("idInterpolationPattern", {
    idInterpolationPattern: "[folder].[name].[sha512:contenthash:hex:6]",
  });
});

test.skip("idInterpolationPattern default", function () {
  transformAndCheck("idInterpolationPattern");
});

test.skip("GH #2663", function () {
  const filePath = path.join(__dirname, "fixtures", `2663.js`);
  const messages: ExtractedMessageDescriptor[] = [];
  const meta = {};

  /*
  const { code } = transformFileSync(filePath, {
    presets: ['@babel/preset-env', '@babel/preset-react'],
    plugins: [
      [
        plugin,
        {
          pragma: '@react-intl',
          onMsgExtracted(_, msgs) {
            messages.push(...msgs)
          },
          onMetaExtracted(_, m) {
            Object.assign(meta, m)
          },
        } as Options,
        Date.now() + '' + ++cacheBust,
      ],
    ],
  })!*/

  const code = "tbd";
  expect({
    data: { messages, meta },
    code: code?.trim(),
  }).toMatchSnapshot();
});

test.skip("overrideIdFn", function () {
  transformAndCheck("overrideIdFn", {
    overrideIdFn: (
      id?: string,
      defaultMessage?: string,
      description?: string,
      filePath?: string
    ) => {
      const filename = path.basename(filePath!);
      return `${filename}.${id}.${
        defaultMessage!.length
      }.${typeof description}`;
    },
  });
});
test.skip("removeDefaultMessage", function () {
  transformAndCheck("removeDefaultMessage", {
    removeDefaultMessage: true,
  });
});
test.skip("removeDefaultMessage + overrideIdFn", function () {
  transformAndCheck("removeDefaultMessage", {
    removeDefaultMessage: true,
    overrideIdFn: (
      id?: string,
      defaultMessage?: string,
      description?: string,
      filePath?: string
    ) => {
      const filename = path.basename(filePath!);
      return `${filename}.${id}.${
        defaultMessage!.length
      }.${typeof description}`;
    },
  });
});
test.skip("preserveWhitespace", function () {
  transformAndCheck("preserveWhitespace", {
    preserveWhitespace: true,
  });
});

test("extractSourceLocation", function () {
  const { data, code } = transformAndCheck("extractSourceLocation", {
    extractSourceLocation: true,
  });

  expect(code).toMatchInlineSnapshot(`
    "import React, { Component } from 'react';
    import { FormattedMessage } from 'react-intl';
    export default class Foo extends Component {
        render() {
            return /*#__PURE__*/ React.createElement(FormattedMessage, {
                id: "foo.bar.baz",
                defaultMessage: "Hello World!"
            });
        }
    }"
  `);

  expect(data.messages).toMatchInlineSnapshot(`
    [
      {
        "defaultMessage": "Hello World!",
        "id": "foo.bar.baz",
        "loc": {
          "end": {
            "col": 78,
            "line": 6,
          },
          "file": "${getFixturePath("extractSourceLocation.js")}",
          "start": {
            "col": 11,
            "line": 6,
          },
        },
      },
    ]
  `);
  expect(data.meta).toMatchInlineSnapshot(`{}`);
  /*
  const filePath = path.join(__dirname, "fixtures", "extractSourceLocation.js");
  const messages: ExtractedMessageDescriptor[] = [];
  const meta = {};

  const { code } = transform(filePath, undefined, {
    pragma: "@react-intl",
    extractSourceLocation: true,
  });
  expect(code?.trim()).toMatchSnapshot();
  expect(messages).toMatchSnapshot([
    {
      file: expect.any(String),
    },
  ]);
  expect(meta).toMatchSnapshot();*/
});

test.skip("Properly throws parse errors", () => {
  expect(() =>
    transform(path.join(__dirname, "fixtures", "icuSyntax.js"))
  ).toThrow("SyntaxError: MALFORMED_ARGUMENT");
});

test.skip("skipExtractionFormattedMessage", function () {
  transformAndCheck("skipExtractionFormattedMessage");
});

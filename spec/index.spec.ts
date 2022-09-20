import { test } from "@jest/globals";
import * as path from "path";
import {
  ExtractedMessageDescriptor,
  transform,
  transformAndCheck,
} from "./transform";

test.skip("additionalComponentNames", function () {
  transformAndCheck("additionalComponentNames", {
    additionalComponentNames: ["CustomMessage"],
  });
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
test.skip("FormattedMessage", function () {
  transformAndCheck("FormattedMessage");
});
test.skip("inline", function () {
  transformAndCheck("inline");
});
test.skip("templateLiteral", function () {
  transformAndCheck("templateLiteral");
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

test.skip("extractSourceLocation", function () {
  const filePath = path.join(__dirname, "fixtures", "extractSourceLocation.js");
  const messages: ExtractedMessageDescriptor[] = [];
  const meta = {};

  const { code } = transform(filePath, undefined, {
    pragma: "@react-intl",
    extractSourceLocation: true,
    onMsgExtracted(_, msgs) {
      messages.push(...msgs);
    },
    onMetaExtracted(_, m) {
      Object.assign(meta, m);
    },
  });
  expect(code?.trim()).toMatchSnapshot();
  expect(messages).toMatchSnapshot([
    {
      file: expect.any(String),
    },
  ]);
  expect(meta).toMatchSnapshot();
});

test.skip("Properly throws parse errors", () => {
  expect(() =>
    transform(path.join(__dirname, "fixtures", "icuSyntax.js"))
  ).toThrow("SyntaxError: MALFORMED_ARGUMENT");
});

test.skip("skipExtractionFormattedMessage", function () {
  transformAndCheck("skipExtractionFormattedMessage");
});

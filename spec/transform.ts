import * as path from "path";
import { Options as swcOptions, transformSync } from "@swc/core";
import * as fs from 'fs';

const pluginBinary = path.resolve(
  __dirname,
  "../target/wasm32-wasi/debug/swc_plugin_formatjs.wasm"
);

let cacheBust = 1

/*
function transform(
  filePath: string,
  options: Options = {},
  { multiplePasses = false } = {}
) {
  function getPluginConfig() {
    return [plugin, options, Date.now() + '' + ++cacheBust]
  }

  return transformFileSync(filePath, {
    presets: [
      [
        '@babel/preset-env',
        {
          targets: {
            node: '14',
            esmodules: true,
          },
          modules: false,
          useBuiltIns: false,
          ignoreBrowserslistConfig: true,
        },
      ],
      '@babel/preset-react',
    ],
    plugins: multiplePasses
      ? [getPluginConfig(), getPluginConfig()]
      : [getPluginConfig()],
  })!
}*/

export interface MessageDescriptor {
  id: string
  defaultMessage?: string
  description?: string
}

export interface SourceLocation {
  //TBD
}

export interface Options {
  overrideIdFn?: (
    id?: string,
    defaultMessage?: string,
    description?: string,
    filePath?: string
  ) => string
  onMsgExtracted?: (filePath: string, msgs: MessageDescriptor[]) => void
  onMetaExtracted?: (filePath: string, meta: Record<string, string>) => void
  idInterpolationPattern?: string
  removeDefaultMessage?: boolean
  additionalComponentNames?: string[]
  additionalFunctionNames?: string[]
  pragma?: string
  extractSourceLocation?: boolean
  ast?: boolean
  preserveWhitespace?: boolean
}

export type ExtractedMessageDescriptor = MessageDescriptor &
  Partial<SourceLocation> & { file?: string };

export const transform = (
  filePath: string,
  transformOptions?: swcOptions,
  pluginOptions?: any,
) => {
  const code = fs.readFileSync(filePath, 'utf-8');

  const options: swcOptions = {
    filename: filePath,
    jsc: {
      parser: {
        syntax: "ecmascript",
        jsx: true,
      },
      target: "es2022",
      preserveAllComments: true,
    },
    isModule: transformOptions?.isModule ?? true,
    module: {
      type: "es6",
      strict: !!transformOptions?.isModule ?? false,
    },
  };

  if (process.env.SWC_TRANSFORM_CUSTOM === "1") {
    const { transformSync } = require("../../index");
    return transformSync(
      code,
      true,
      Buffer.from(JSON.stringify(options)),
      Buffer.from(
        JSON.stringify({
          ...pluginOptions,
          pragma: '@react-intl',
          debugInitialCoverageComment: true,
        })
      )
    ).code;
  }

  options.jsc!.experimental = {
    plugins: [
      [
        pluginBinary,
        {
          pragma: '@react-intl',
          ...pluginOptions,
        },
      ],
    ],
  };

  return transformSync(code, options);
};

export function transformAndCheck(fn: string, opts: Options = {}) {
  const filePath = path.join(__dirname, 'fixtures', `${fn}.js`)
  const messages: ExtractedMessageDescriptor[] = []
  const meta = {}
  const { code } = transform(filePath, undefined, {
    pragma: '@react-intl',
    ...opts,
    onMsgExtracted(_, msgs) {
      messages.push(...msgs)
    },
    onMetaExtracted(_, m) {
      Object.assign(meta, m)
    },
  })


  return {
    data: { messages, meta },
    code: code?.trim(),
  };
}
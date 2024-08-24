import { FlatCompat } from '@eslint/eslintrc'
import pluginJs from '@eslint/js'
import pluginStylistic from '@stylistic/eslint-plugin-js'
import pluginImport from 'eslint-plugin-import'
import pluginJest from 'eslint-plugin-jest'
import globals from 'globals'
import path from 'path'
import tsEsLint from 'typescript-eslint'
import { fileURLToPath } from 'url'

// mimic CommonJS variables -- not needed if using CommonJS
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: pluginJs.configs.recommended
})

export default [
  {
    languageOptions: {
      globals: globals.node,
      parserOptions: {
        project: true,
        tsconfigRootDir: __dirname,
      }
    },
  },
  ...compat.extends('standard'),
  {
    files: ['tests/**/*.{js,ts}'],
    ...pluginJest.configs['flat/recommended'],
    rules: {
      'jest/expect-expect': [
        'warn',
        { assertFunctionNames: ['expect', 'fc.assert'] }
      ]
    }
  },
  {
    files: ['tests/**/*.{js,ts}'],
    ...pluginStylistic.configs['all-flat'],
    rules: {
      'dot-notation': /*
        conflicts with "noPropertyAccessFromIndexSignature" typescript
        compiler option
      */ 'off',
      'max-len': ['warn', { code: 79, ignoreComments: true }],
      'operator-linebreak': ['warn', 'before']
    }
  },
  {
    ...pluginImport.configs.typescript,
    rules: {
      ...pluginImport.configs.typescript.rules,
      'import/order': [
        'warn',
        {
          'groups': [
            'builtin',
            'external',
            'internal',
            'parent',
            'sibling',
            'index',
            'object',
            'type'
          ],
          'alphabetize': {
            order: 'asc',
            orderImportKind: 'asc'
          },
          'newlines-between': 'never'
        }
      ],
      'import/no-self-import': 'warn',
      'import/no-mutable-exports': 'warn',
      'import/no-commonjs': 'warn',
    }
  },
  ...tsEsLint.configs.strictTypeChecked,
  {
    rules: {
      '@typescript-eslint/no-non-null-assertion': 'off'
    }
  }
]

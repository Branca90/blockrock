import vue from 'eslint-plugin-vue';
import ts from '@typescript-eslint/eslint-plugin';
import parser from '@typescript-eslint/parser';
import vueParser from 'vue-eslint-parser';

export default [
  {
    ignores: ['**/dist/**', '**/dist-ssr/**', '**/coverage/**'],
  },
  {
    files: ['**/*.{js,ts,vue}'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: parser,
        sourceType: 'module',
        ecmaVersion: 'latest',
      },
    },
    plugins: {
      vue,
      '@typescript-eslint': ts,
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      '@typescript-eslint/no-unused-vars': ['error'],
    },
  },
];

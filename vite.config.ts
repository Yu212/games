import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasmPack from 'vite-plugin-wasm-pack'
import {comlink} from 'vite-plugin-comlink'

export default defineConfig({
  plugins: [react(), wasmPack(['./rust']), comlink()],
  worker: {
    plugins: [comlink()]
  },
  server: {
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp"
    }
  }
})

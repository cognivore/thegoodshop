// pages/+config.ts
import vikeReact from 'vike-react/config'
import type { Config } from 'vike/types'
import Layout from '../layouts/LayoutDefault'

export default {
  Layout,
  title: 'The Good Shop',
  description: 'The Good Shop frontend',
  // In the V1 design, values returned as "data" are passed by default,
  // but we can explicitly pass it if desired:
  passToClient: ['data'],
  prerender: {
    partial: false,
    noExtraDir: false,
    parallel: 4,
    disableAutoRun: false
  },
  extends: [vikeReact],
} satisfies Config

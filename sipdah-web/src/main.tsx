import { RouterProvider } from '@tanstack/react-router'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Toaster } from 'sonner'

import { router } from '@/router-config'
import './index.css'
import { ThemeProvider } from '@/pages/landing/components/theme-provider'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ThemeProvider>
      <Toaster />
      <RouterProvider router={router} />
    </ThemeProvider>
  </StrictMode>
)

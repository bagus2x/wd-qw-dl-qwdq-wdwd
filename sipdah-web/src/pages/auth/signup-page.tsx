import { createRoute, HeadContent } from '@tanstack/react-router'

import { SignUpForm } from '@/pages/auth/components/signup-form'
import { rootRoute } from '@/router-config'

export const signUpRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/signup',
  component: () => (
    <>
      <HeadContent />
      <SignUpPage />
    </>
  ),
})

export function SignUpPage() {
  return (
    <main className='mx-auto p-4 md:py-16 w-full flex justify-center'>
      <SignUpForm className='max-w-96 w-full' />
    </main>
  )
}

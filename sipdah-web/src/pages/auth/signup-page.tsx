import { SignUpForm } from '@/pages/auth/components/signup-form'
import { createLazyRoute, HeadContent } from '@tanstack/react-router'

export const signUpLazyRoute = createLazyRoute('/signup')({
  component: () => (
    <>
      <HeadContent />
      <SignUpPage />
    </>
  ),
})

export function SignUpPage() {
  return (
    <main className='mx-auto p-4 w-full flex justify-center'>
      <SignUpForm className='max-w-96 w-full' />
    </main>
  )
}

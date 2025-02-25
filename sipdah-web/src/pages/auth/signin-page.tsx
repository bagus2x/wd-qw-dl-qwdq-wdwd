import { SignInForm } from '@/pages/auth/components/signin-form'
import { createLazyRoute, HeadContent } from '@tanstack/react-router'

export const signInLazyRoute = createLazyRoute('/signin')({
  component: () => (
    <>
      <HeadContent />
      <SignInPage />
    </>
  ),
})

export function SignInPage() {
  return (
    <main className='mx-auto p-4 w-full flex justify-center'>
      <SignInForm className='max-w-96 w-full' />
    </main>
  )
}

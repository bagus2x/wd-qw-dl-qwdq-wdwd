import { useIsSignedIn } from '@/data/auth-hooks'
import { SignInForm } from '@/pages/auth/components/signin-form'
import { rootRoute } from '@/router-config'
import { createRoute, HeadContent } from '@tanstack/react-router'

export const signInRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/signin',
  component: () => (
    <>
      <HeadContent />
      <SignInPage />
    </>
  ),
})

export function SignInPage() {
  const isSignedIn = useIsSignedIn()
  console.log(isSignedIn)

  return (
    <main className='mx-auto p-4 w-full flex justify-center'>
      <SignInForm className='max-w-96 w-full' />
    </main>
  )
}

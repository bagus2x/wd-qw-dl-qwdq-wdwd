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
  return <main>Hello Sign In</main>
}

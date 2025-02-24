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
  return <main>Hello Sign Up</main>
}

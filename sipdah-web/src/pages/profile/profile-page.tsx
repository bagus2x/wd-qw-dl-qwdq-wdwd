import { createRoute, HeadContent, useRouter } from '@tanstack/react-router'

import { Button } from '@/components/ui/button'
import { useSignOut } from '@/data/auth-hooks'
import { rootRoute } from '@/router-config'

export const profileRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/profile',
  component: () => (
    <>
      <HeadContent />
      <ProfilePage />
    </>
  ),
  head() {
    return {
      meta: [{ title: 'Sipdah' }],
    }
  },
})

export function ProfilePage() {
  const signOut = useSignOut()
  const router = useRouter()

  const handleSignout = () => {
    signOut.mutate(undefined, {
      onSuccess: () => {
        router.navigate({ to: '/', replace: true })
      },
    })
  }

  return (
    <main className='w-full flex flex-col items-center'>
      Profile page
      <Button onClick={handleSignout}>Sign out</Button>
    </main>
  )
}

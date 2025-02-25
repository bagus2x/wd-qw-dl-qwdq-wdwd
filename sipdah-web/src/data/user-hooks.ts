import { useQuery } from '@tanstack/react-query'

import { getCurrentUser } from '@/data/user-service'

export const userHooksKeys = {
  current_user: ['current_user'],
}

export const useCurrentUser = () => {
  return useQuery({
    queryKey: userHooksKeys['current_user'],
    queryFn: getCurrentUser,
  })
}

import { useRouter, useRoute } from 'vue-router'
import type { RouteLocationRaw } from 'vue-router'

export const useNavigation = () => {
  const router = useRouter()
  const route = useRoute()

  const navigateTo = (to: RouteLocationRaw) => {
    router.push(to)
  }

  const isActiveRoute = (routeName: string | symbol) => {
    return route.name === routeName
  }

  return {
    navigateTo,
    isActiveRoute,
    currentRoute: route
  }
}


import {
  IconCategoryFilled,
  IconMenu,
  IconPuzzle,
  IconUser,
  IconUserFilled,
} from '@tabler/icons-react'
import { useState } from 'react'

import { Button, buttonVariants } from '@/components/ui/button'
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
} from '@/components/ui/navigation-menu'
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from '@/components/ui/sheet'
import { useIsSignedIn } from '@/data/auth-hooks'
import { LogoIcon } from '@/pages/landing/components/icons'
import { ModeToggle } from '@/pages/landing/components/mode-toggle'
import { Link } from '@tanstack/react-router'

interface RouteProps {
  href: string
  label: string
}

const routeList: RouteProps[] = [
  {
    href: '#features',
    label: 'Features',
  },
  {
    href: '#testimonials',
    label: 'Testimonials',
  },
  {
    href: '#pricing',
    label: 'Pricing',
  },
  {
    href: '#faq',
    label: 'FAQ',
  },
]

export const Navbar = () => {
  const [isOpen, setIsOpen] = useState<boolean>(false)
  const isSignedIn = useIsSignedIn()

  return (
    <header className='sticky border-b-[1px] top-0 z-40 w-full bg-white dark:border-b-slate-700 dark:bg-background'>
      <NavigationMenu className='mx-auto'>
        <NavigationMenuList className='container h-14 px-4 w-screen flex justify-between '>
          <NavigationMenuItem className='font-bold flex'>
            <a rel='noreferrer noopener' href='/' className='ml-2 font-bold text-xl flex'>
              <LogoIcon />
              Sipdah
            </a>
          </NavigationMenuItem>

          {/* Mobile */}
          <span className='flex md:hidden'>
            <ModeToggle />

            <Sheet open={isOpen} onOpenChange={setIsOpen}>
              <SheetTrigger className='px-2'>
                <IconMenu className='flex md:hidden h-5 w-5' onClick={() => setIsOpen(true)}>
                  <span className='sr-only'>Menu Icon</span>
                </IconMenu>
              </SheetTrigger>

              <SheetContent side={'left'}>
                <SheetHeader>
                  <SheetTitle className='font-bold text-xl'>Shadcn/React</SheetTitle>
                </SheetHeader>
                <nav className='flex flex-col justify-center items-center gap-2 mt-4'>
                  {routeList.map(({ href, label }: RouteProps) => (
                    <a
                      rel='noreferrer noopener'
                      key={label}
                      href={href}
                      onClick={() => setIsOpen(false)}
                      className={buttonVariants({ variant: 'ghost' })}
                    >
                      {label}
                    </a>
                  ))}
                  {!isSignedIn && (
                    <Button asChild variant='secondary'>
                      <Link to='/signin'>Login</Link>
                    </Button>
                  )}
                  {isSignedIn && (
                    <Button asChild variant='secondary' size='icon'>
                      <Link to='/profile'>
                        <IconUserFilled />
                        <span className='sr-only'>Profile</span>
                      </Link>
                    </Button>
                  )}
                  {isSignedIn && (
                    <Button asChild variant='secondary' size='icon'>
                      <Link to='/profile'>
                        <IconCategoryFilled />
                        <span className='sr-only'>Dashboard</span>
                      </Link>
                    </Button>
                  )}
                </nav>
              </SheetContent>
            </Sheet>
          </span>

          {/* Desktop */}
          <nav className='hidden md:flex gap-2'>
            {routeList.map((route: RouteProps, i) => (
              <a
                rel='noreferrer noopener'
                href={route.href}
                key={i}
                className={`text-[17px] ${buttonVariants({
                  variant: 'ghost',
                })}`}
              >
                {route.label}
              </a>
            ))}
          </nav>

          <div className='hidden md:flex gap-2'>
            {!isSignedIn && (
              <Button asChild variant='secondary'>
                <Link to='/signin'>Login</Link>
              </Button>
            )}
            {isSignedIn && (
              <Button asChild variant='secondary' size='icon'>
                <Link to='/profile'>
                  <IconUser />
                  <span className='sr-only'>Profile</span>
                </Link>
              </Button>
            )}
            {isSignedIn && (
              <Button asChild variant='secondary' size='icon'>
                <Link to='/profile'>
                  <IconPuzzle />
                  <span className='sr-only'>Dashboard</span>
                </Link>
              </Button>
            )}
            <ModeToggle />
          </div>
        </NavigationMenuList>
      </NavigationMenu>
    </header>
  )
}

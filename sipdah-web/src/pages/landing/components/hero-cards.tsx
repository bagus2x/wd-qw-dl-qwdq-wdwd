import {
  IconBrandGithubFilled,
  IconBrandLinkedinFilled,
  IconCheck,
  IconLamp,
} from '@tabler/icons-react'

import { Button, buttonVariants } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  CardFooter,
} from '@/components/ui/card'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { AvatarImage } from '@radix-ui/react-avatar'
import { Badge } from '@/components/ui/badge'

export const HeroCards = () => {
  return (
    <div className='hidden lg:flex flex-row flex-wrap gap-8 relative w-[700px] h-[500px]'>
      {/* Testimonial */}
      <Card className='absolute w-[340px] -top-[15px] drop-shadow-xl shadow-black/10 dark:shadow-white/10'>
        <CardHeader className='flex flex-row items-center gap-4 pb-2'>
          <Avatar>
            <AvatarImage alt='' src='https://github.com/shadcn.png' />
            <AvatarFallback>SH</AvatarFallback>
          </Avatar>

          <div className='flex flex-col'>
            <CardTitle className='text-lg'>John Doe React</CardTitle>
            <CardDescription>@john_doe</CardDescription>
          </div>
        </CardHeader>
        <CardContent>This platform is awesome!</CardContent>
      </Card>

      {/* Team */}
      <Card className='absolute right-[20px] top-4 w-80 flex flex-col justify-center items-center drop-shadow-xl shadow-black/10 dark:shadow-white/10'>
        <CardHeader className='mt-8 flex justify-center items-center pb-2'>
          <img
            src='https://i.pravatar.cc/150?img=58'
            alt='user avatar'
            className='absolute grayscale-[0%] -top-12 rounded-full w-24 h-24 aspect-square object-cover'
          />
          <CardTitle className='text-center'>Leo Miranda</CardTitle>
          <CardDescription className='font-normal text-primary'>Frontend Developer</CardDescription>
        </CardHeader>

        <CardContent className='text-center pb-2'>
          <p>
            I really enjoy transforming ideas into functional software that exceeds expectations
          </p>
        </CardContent>

        <CardFooter>
          <div>
            <a
              rel='noreferrer noopener'
              href='https://github.com/leoMirandaa'
              target='_blank'
              className={buttonVariants({
                variant: 'ghost',
                size: 'sm',
              })}
            >
              <span className='sr-only'>Github icon</span>
              <IconBrandGithubFilled className='w-5 h-5' />
            </a>
            <a
              rel='noreferrer noopener'
              href='https://twitter.com/leo_mirand4'
              target='_blank'
              className={buttonVariants({
                variant: 'ghost',
                size: 'sm',
              })}
            >
              <span className='sr-only'>X icon</span>
              <svg
                role='img'
                viewBox='0 0 24 24'
                xmlns='http://www.w3.org/2000/svg'
                className='fill-foreground w-5 h-5'
              >
                <title>X</title>
                <path d='M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z' />
              </svg>
            </a>

            <a
              rel='noreferrer noopener'
              href='https://www.linkedin.com/in/leopoldo-miranda/'
              target='_blank'
              className={buttonVariants({
                variant: 'ghost',
                size: 'sm',
              })}
            >
              <span className='sr-only'>Linkedin icon</span>
              <IconBrandLinkedinFilled size='20' />
            </a>
          </div>
        </CardFooter>
      </Card>

      {/* Pricing */}
      <Card className='absolute top-[150px] left-[50px] w-72  drop-shadow-xl shadow-black/10 dark:shadow-white/10'>
        <CardHeader>
          <CardTitle className='flex item-center justify-between'>
            Basic
            <Badge variant='secondary' className='text-sm text-primary'>
              Most popular
            </Badge>
          </CardTitle>
          <div>
            <span className='text-3xl font-bold'>IDR 5.000.000</span>
            <span className='text-muted-foreground'> /month</span>
          </div>

          <CardDescription>
            Get started with essential features, perfect for individuals and small teams.
          </CardDescription>
        </CardHeader>

        <CardContent>
          <Button className='w-full'>Create project</Button>
        </CardContent>

        <hr className='w-4/5 m-auto mb-4' />

        <CardFooter className='flex'>
          <div className='space-y-4'>
            {['Forum for discussion', '4x Open Ticket'].map((benefit: string) => (
              <span key={benefit} className='flex'>
                <IconCheck className='text-green-500' /> <h3 className='ml-2'>{benefit}</h3>
              </span>
            ))}
          </div>
        </CardFooter>
      </Card>

      {/* Service */}
      <Card className='absolute w-[350px] -right-[10px] bottom-[35px]  drop-shadow-xl shadow-black/10 dark:shadow-white/10'>
        <CardHeader className='space-y-1 flex md:flex-row justify-start items-start gap-4'>
          <div className='mt-1 bg-primary/20 p-1 rounded-2xl'>
            <IconLamp />
          </div>
          <div>
            <CardTitle>Light & dark mode</CardTitle>
            <CardDescription className='text-md mt-2'>
              Lorem ipsum dolor sit amet consect adipisicing elit. Consectetur natusm.
            </CardDescription>
          </div>
        </CardHeader>
      </Card>
    </div>
  )
}

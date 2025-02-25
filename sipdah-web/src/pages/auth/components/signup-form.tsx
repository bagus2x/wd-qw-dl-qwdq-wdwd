import { zodResolver } from '@hookform/resolvers/zod'
import { IconBrandGoogleFilled, IconLoader } from '@tabler/icons-react'
import { Link } from '@tanstack/react-router'
import { useForm } from 'react-hook-form'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useSignUp } from '@/data/auth-hooks'
import { Api } from '@/data/common'
import { isAxiosError } from '@/lib/axios'
import { cn } from '@/lib/utils'

const formSchema = z.object({
  name: z.string().min(1).max(64),
  email: z.string().email().min(1).max(64),
  password: z.string().min(1).max(16),
})

export type SignUpFormProps = React.ComponentPropsWithoutRef<'div'>

export function SignUpForm({ className, ...props }: SignUpFormProps) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      email: '',
      password: '',
    },
  })
  const signUp = useSignUp()

  function onSubmit(values: z.infer<typeof formSchema>) {
    signUp.mutate(values, {
      onSuccess: () => {
        toast.success('Account created successfully! You can now log in 🎉🎉', {
          richColors: true,
          position: 'top-center',
        })
      },
      onError: (err) => {
        if (isAxiosError<Api>(err)) {
          const message = err.response?.data.message || err.message
          toast.error(message, { richColors: true, position: 'top-center' })
        } else {
          toast.error(err.message, { richColors: true, position: 'top-center' })
        }
      },
    })
  }

  return (
    <div className={cn('flex flex-col gap-6', className)} {...props}>
      <Card>
        <CardHeader className='text-center'>
          <CardTitle className='text-xl'>Join us today!</CardTitle>
          <CardDescription>Create an account to get started.</CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)}>
              <div className='grid gap-6'>
                <div className='grid gap-6'>
                  <FormField
                    control={form.control}
                    name='name'
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Name</FormLabel>
                        <FormControl>
                          <Input placeholder='Your name' {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name='email'
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Email</FormLabel>
                        <FormControl>
                          <Input placeholder='Your email' {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name='password'
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Password</FormLabel>
                        <FormControl>
                          <Input type='password' placeholder='Create a password' {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <Button type='submit' className='w-full' disabled={signUp.isPending}>
                    {signUp.isPending && <IconLoader className='animate-spin' />} Sign Up
                  </Button>
                </div>
                <div className='text-center text-sm'>
                  Already have an account?{' '}
                  <Link to='/signin' className='underline underline-offset-4'>
                    Log in
                  </Link>
                </div>
              </div>
              <div className='relative text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t after:border-border my-4'>
                <span className='relative z-10 bg-background px-2 text-muted-foreground'>
                  Or continue with
                </span>
              </div>
              <div className='flex flex-col gap-4'>
                <Button variant='outline' className='w-full'>
                  <IconBrandGoogleFilled />
                  Sign Up with Google
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
      <div className='text-balance text-center text-xs text-muted-foreground [&_a]:underline [&_a]:underline-offset-4 [&_a]:hover:text-primary'>
        By clicking continue, you agree to our <a href='#'>Terms of Service</a> and{' '}
        <a href='#'>Privacy Policy</a>.
      </div>
    </div>
  )
}

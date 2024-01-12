import { component$ } from '@builder.io/qwik'
import type { DocumentHead } from '@builder.io/qwik-city'
import { Notification } from '~/components/notification/notification'

export default component$(() => {
    return (
        <>
            <h1>Hallo, Qwik qorld!</h1>
            <Notification
                state="none"
                title="test notif"
                msg="some random message"
            />
        </>
    )
})

export const head: DocumentHead = {
    title: 'MiniYT',
    meta: [
        {
            name: 'Minimal YT client',
            content: 'Minimal YT client to manage your subscriptions',
        },
    ],
}

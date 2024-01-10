import { component$ } from '@builder.io/qwik';
import type { DocumentHead } from '@builder.io/qwik-city';

export default component$(() => {
    return (
        <>
            <h1>Hallo, Qwik qorld!</h1>
        </>
    );
});

export const head: DocumentHead = {
    title: 'MiniYT',
    meta: [
        {
            name: 'description',
            content: 'Qwik site description',
        },
    ],
};

import { Button } from 'styles/Form';
import { Wrapper } from 'styles/Wrapper';
import { ExternalLink } from 'styles/Link';
import { Title, Subtitle } from 'styles/Title';
import { Size } from 'styles/common';

const Playground = () => {
  return (
    <Wrapper $vertical $bordered $padding={Size.Medium}>
      <Wrapper $vertical $bordered $padding={Size.Small}>
        <Title>Title</Title>
        <Subtitle>Subtitle</Subtitle>
      </Wrapper>
      <ExternalLink>ExternalLink</ExternalLink>
      <Button>Button</Button>
    </Wrapper>
  );
};

export default Playground;

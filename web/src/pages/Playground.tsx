import { Button } from 'styles/Button';
import { Card, CardTitle, CardSubtitle, CardContent, CardFooter } from 'styles/Card';
import { Wrapper } from 'styles/Wrapper';
import { Link } from 'styles/Link';

const Playground = () => {
  return (
    <Wrapper>
      <Card>
        <CardTitle>
          <h1>Title</h1>
        </CardTitle>
        <CardSubtitle>
          # <Link href="#">subtitle</Link>
        </CardSubtitle>
        <CardContent>Content</CardContent>
        <CardFooter>
          Footer
          <div>
            <Button>Hello</Button>
            <Button primary>Hello</Button>
          </div>
        </CardFooter>
      </Card>
    </Wrapper>
  );
};

export default Playground;

public class Main {
    public static void main(String[] args) throws Throwable {
        System.out.println("Java FFM C Bindings Demo");
        System.out.println("========================");

        int result1 = (int) MathBindings.addNumbers.invokeExact(10, 20);
        System.out.println("add_numbers(10, 20) = " + result1);

        int result2 = (int) MathBindings.multiplyNumbers.invokeExact(7, 6);
        System.out.println("multiply_numbers(7, 6) = " + result2);
    }
}

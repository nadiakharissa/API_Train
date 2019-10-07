package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object bookingcode
     
    /**
     * <p></p>
     */
    public static Object PNRID
     
    /**
     * <p></p>
     */
    public static Object wagonno
     
    /**
     * <p></p>
     */
    public static Object wagoncode
     
    /**
     * <p></p>
     */
    public static Object seat
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += RunConfiguration.getOverridingParameters()
    
            bookingcode = selectedVariables['bookingcode']
            PNRID = selectedVariables['PNRID']
            wagonno = selectedVariables['wagonno']
            wagoncode = selectedVariables['wagoncode']
            seat = selectedVariables['seat']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}

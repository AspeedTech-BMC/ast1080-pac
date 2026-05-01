#[doc = "Register `OTP_REG32C` reader"]
pub type R = crate::R<OtpReg32cSpec>;
#[doc = "Register `OTP_REG32C` writer"]
pub type W = crate::W<OtpReg32cSpec>;
#[doc = "Field `REGSWINFO11` reader - REG_SW_INFO11"]
pub type Regswinfo11R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO11` writer - REG_SW_INFO11"]
pub type Regswinfo11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO11"]
    #[inline(always)]
    pub fn regswinfo11(&self) -> Regswinfo11R {
        Regswinfo11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO11"]
    #[inline(always)]
    pub fn regswinfo11(&mut self) -> Regswinfo11W<OtpReg32cSpec> {
        Regswinfo11W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE11\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg32c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg32c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg32cSpec;
impl crate::RegisterSpec for OtpReg32cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg32c::R`](R) reader structure"]
impl crate::Readable for OtpReg32cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg32c::W`](W) writer structure"]
impl crate::Writable for OtpReg32cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG32C to value 0x11"]
impl crate::Resettable for OtpReg32cSpec {
    const RESET_VALUE: u32 = 0x11;
}

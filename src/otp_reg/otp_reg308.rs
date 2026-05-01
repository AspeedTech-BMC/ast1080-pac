#[doc = "Register `OTP_REG308` reader"]
pub type R = crate::R<OtpReg308Spec>;
#[doc = "Register `OTP_REG308` writer"]
pub type W = crate::W<OtpReg308Spec>;
#[doc = "Field `REGSWINFO2` reader - REG_SW_INFO2"]
pub type Regswinfo2R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO2` writer - REG_SW_INFO2"]
pub type Regswinfo2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO2"]
    #[inline(always)]
    pub fn regswinfo2(&self) -> Regswinfo2R {
        Regswinfo2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO2"]
    #[inline(always)]
    pub fn regswinfo2(&mut self) -> Regswinfo2W<OtpReg308Spec> {
        Regswinfo2W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg308::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg308::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg308Spec;
impl crate::RegisterSpec for OtpReg308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg308::R`](R) reader structure"]
impl crate::Readable for OtpReg308Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg308::W`](W) writer structure"]
impl crate::Writable for OtpReg308Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG308 to value 0x02"]
impl crate::Resettable for OtpReg308Spec {
    const RESET_VALUE: u32 = 0x02;
}

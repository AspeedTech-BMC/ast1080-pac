#[doc = "Register `OTP_REG1E8` reader"]
pub type R = crate::R<OtpReg1e8Spec>;
#[doc = "Register `OTP_REG1E8` writer"]
pub type W = crate::W<OtpReg1e8Spec>;
#[doc = "Field `REGCALIPTRARUNTIMESVN2` reader - REG_CALIPTRA_RUNTIME_SVN_2"]
pub type Regcaliptraruntimesvn2R = crate::FieldReader<u32>;
#[doc = "Field `REGCALIPTRARUNTIMESVN2` writer - REG_CALIPTRA_RUNTIME_SVN_2"]
pub type Regcaliptraruntimesvn2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_2"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn2(&self) -> Regcaliptraruntimesvn2R {
        Regcaliptraruntimesvn2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_2"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn2(&mut self) -> Regcaliptraruntimesvn2W<OtpReg1e8Spec> {
        Regcaliptraruntimesvn2W::new(self, 0)
    }
}
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1e8Spec;
impl crate::RegisterSpec for OtpReg1e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1e8::R`](R) reader structure"]
impl crate::Readable for OtpReg1e8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1e8::W`](W) writer structure"]
impl crate::Writable for OtpReg1e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1E8 to value 0"]
impl crate::Resettable for OtpReg1e8Spec {}

#[doc = "Register `OTP_REG1E4` reader"]
pub type R = crate::R<OtpReg1e4Spec>;
#[doc = "Register `OTP_REG1E4` writer"]
pub type W = crate::W<OtpReg1e4Spec>;
#[doc = "Field `REGCALIPTRARUNTIMESVN1` reader - REG_CALIPTRA_RUNTIME_SVN_1"]
pub type Regcaliptraruntimesvn1R = crate::FieldReader<u32>;
#[doc = "Field `REGCALIPTRARUNTIMESVN1` writer - REG_CALIPTRA_RUNTIME_SVN_1"]
pub type Regcaliptraruntimesvn1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_1"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn1(&self) -> Regcaliptraruntimesvn1R {
        Regcaliptraruntimesvn1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_1"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn1(&mut self) -> Regcaliptraruntimesvn1W<OtpReg1e4Spec> {
        Regcaliptraruntimesvn1W::new(self, 0)
    }
}
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1e4Spec;
impl crate::RegisterSpec for OtpReg1e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1e4::R`](R) reader structure"]
impl crate::Readable for OtpReg1e4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1e4::W`](W) writer structure"]
impl crate::Writable for OtpReg1e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1E4 to value 0"]
impl crate::Resettable for OtpReg1e4Spec {}

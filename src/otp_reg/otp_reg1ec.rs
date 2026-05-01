#[doc = "Register `OTP_REG1EC` reader"]
pub type R = crate::R<OtpReg1ecSpec>;
#[doc = "Register `OTP_REG1EC` writer"]
pub type W = crate::W<OtpReg1ecSpec>;
#[doc = "Field `REGCALIPTRARUNTIMESVN3` reader - REG_CALIPTRA_RUNTIME_SVN_3"]
pub type Regcaliptraruntimesvn3R = crate::FieldReader<u32>;
#[doc = "Field `REGCALIPTRARUNTIMESVN3` writer - REG_CALIPTRA_RUNTIME_SVN_3"]
pub type Regcaliptraruntimesvn3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_3"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn3(&self) -> Regcaliptraruntimesvn3R {
        Regcaliptraruntimesvn3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_3"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn3(&mut self) -> Regcaliptraruntimesvn3W<OtpReg1ecSpec> {
        Regcaliptraruntimesvn3W::new(self, 0)
    }
}
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1ecSpec;
impl crate::RegisterSpec for OtpReg1ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1ec::R`](R) reader structure"]
impl crate::Readable for OtpReg1ecSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1ec::W`](W) writer structure"]
impl crate::Writable for OtpReg1ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1EC to value 0"]
impl crate::Resettable for OtpReg1ecSpec {}

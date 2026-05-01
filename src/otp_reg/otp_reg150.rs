#[doc = "Register `OTP_REG150` reader"]
pub type R = crate::R<OtpReg150Spec>;
#[doc = "Register `OTP_REG150` writer"]
pub type W = crate::W<OtpReg150Spec>;
#[doc = "Field `REGREGIONUSR2REN` reader - REG_REGION_USR2_REN"]
pub type Regregionusr2renR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR2REN` writer - REG_REGION_USR2_REN"]
pub type Regregionusr2renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR2WEN` reader - REG_REGION_USR2_WEN"]
pub type Regregionusr2wenR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR2WEN` writer - REG_REGION_USR2_WEN"]
pub type Regregionusr2wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR2LOCK` reader - REG_REGION_USR2_LOCK"]
pub type Regregionusr2lockR = crate::BitReader;
#[doc = "Field `REGREGIONUSR2LOCK` writer - REG_REGION_USR2_LOCK"]
pub type Regregionusr2lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_USR2_REN"]
    #[inline(always)]
    pub fn regregionusr2ren(&self) -> Regregionusr2renR {
        Regregionusr2renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR2_WEN"]
    #[inline(always)]
    pub fn regregionusr2wen(&self) -> Regregionusr2wenR {
        Regregionusr2wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_USR2_LOCK"]
    #[inline(always)]
    pub fn regregionusr2lock(&self) -> Regregionusr2lockR {
        Regregionusr2lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_USR2_REN"]
    #[inline(always)]
    pub fn regregionusr2ren(&mut self) -> Regregionusr2renW<OtpReg150Spec> {
        Regregionusr2renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR2_WEN"]
    #[inline(always)]
    pub fn regregionusr2wen(&mut self) -> Regregionusr2wenW<OtpReg150Spec> {
        Regregionusr2wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_USR2_LOCK"]
    #[inline(always)]
    pub fn regregionusr2lock(&mut self) -> Regregionusr2lockW<OtpReg150Spec> {
        Regregionusr2lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg150Spec;
impl crate::RegisterSpec for OtpReg150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg150::R`](R) reader structure"]
impl crate::Readable for OtpReg150Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg150::W`](W) writer structure"]
impl crate::Writable for OtpReg150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG150 to value 0xffff"]
impl crate::Resettable for OtpReg150Spec {
    const RESET_VALUE: u32 = 0xffff;
}

#[doc = "Register `OTP_REG158` reader"]
pub type R = crate::R<OtpReg158Spec>;
#[doc = "Register `OTP_REG158` writer"]
pub type W = crate::W<OtpReg158Spec>;
#[doc = "Field `REGREGIONUSR3REN` reader - REG_REGION_USR3_REN"]
pub type Regregionusr3renR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR3REN` writer - REG_REGION_USR3_REN"]
pub type Regregionusr3renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR3WEN` reader - REG_REGION_USR3_WEN"]
pub type Regregionusr3wenR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR3WEN` writer - REG_REGION_USR3_WEN"]
pub type Regregionusr3wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR3LOCK` reader - REG_REGION_USR3_LOCK"]
pub type Regregionusr3lockR = crate::BitReader;
#[doc = "Field `REGREGIONUSR3LOCK` writer - REG_REGION_USR3_LOCK"]
pub type Regregionusr3lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_USR3_REN"]
    #[inline(always)]
    pub fn regregionusr3ren(&self) -> Regregionusr3renR {
        Regregionusr3renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR3_WEN"]
    #[inline(always)]
    pub fn regregionusr3wen(&self) -> Regregionusr3wenR {
        Regregionusr3wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_USR3_LOCK"]
    #[inline(always)]
    pub fn regregionusr3lock(&self) -> Regregionusr3lockR {
        Regregionusr3lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_USR3_REN"]
    #[inline(always)]
    pub fn regregionusr3ren(&mut self) -> Regregionusr3renW<OtpReg158Spec> {
        Regregionusr3renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR3_WEN"]
    #[inline(always)]
    pub fn regregionusr3wen(&mut self) -> Regregionusr3wenW<OtpReg158Spec> {
        Regregionusr3wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_USR3_LOCK"]
    #[inline(always)]
    pub fn regregionusr3lock(&mut self) -> Regregionusr3lockW<OtpReg158Spec> {
        Regregionusr3lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg158Spec;
impl crate::RegisterSpec for OtpReg158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg158::R`](R) reader structure"]
impl crate::Readable for OtpReg158Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg158::W`](W) writer structure"]
impl crate::Writable for OtpReg158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG158 to value 0xffff"]
impl crate::Resettable for OtpReg158Spec {
    const RESET_VALUE: u32 = 0xffff;
}

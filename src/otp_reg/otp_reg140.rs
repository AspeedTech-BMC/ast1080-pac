#[doc = "Register `OTP_REG140` reader"]
pub type R = crate::R<OtpReg140Spec>;
#[doc = "Register `OTP_REG140` writer"]
pub type W = crate::W<OtpReg140Spec>;
#[doc = "Field `REGREGIONUSR0REN` reader - REG_REGION_USR0_REN"]
pub type Regregionusr0renR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR0REN` writer - REG_REGION_USR0_REN"]
pub type Regregionusr0renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR0WEN` reader - REG_REGION_USR0_WEN"]
pub type Regregionusr0wenR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR0WEN` writer - REG_REGION_USR0_WEN"]
pub type Regregionusr0wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR0LOCK` reader - REG_REGION_USR0_LOCK"]
pub type Regregionusr0lockR = crate::BitReader;
#[doc = "Field `REGREGIONUSR0LOCK` writer - REG_REGION_USR0_LOCK"]
pub type Regregionusr0lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_USR0_REN"]
    #[inline(always)]
    pub fn regregionusr0ren(&self) -> Regregionusr0renR {
        Regregionusr0renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR0_WEN"]
    #[inline(always)]
    pub fn regregionusr0wen(&self) -> Regregionusr0wenR {
        Regregionusr0wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_USR0_LOCK"]
    #[inline(always)]
    pub fn regregionusr0lock(&self) -> Regregionusr0lockR {
        Regregionusr0lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_USR0_REN"]
    #[inline(always)]
    pub fn regregionusr0ren(&mut self) -> Regregionusr0renW<OtpReg140Spec> {
        Regregionusr0renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR0_WEN"]
    #[inline(always)]
    pub fn regregionusr0wen(&mut self) -> Regregionusr0wenW<OtpReg140Spec> {
        Regregionusr0wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_USR0_LOCK"]
    #[inline(always)]
    pub fn regregionusr0lock(&mut self) -> Regregionusr0lockW<OtpReg140Spec> {
        Regregionusr0lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg140Spec;
impl crate::RegisterSpec for OtpReg140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg140::R`](R) reader structure"]
impl crate::Readable for OtpReg140Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg140::W`](W) writer structure"]
impl crate::Writable for OtpReg140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG140 to value 0xffff"]
impl crate::Resettable for OtpReg140Spec {
    const RESET_VALUE: u32 = 0xffff;
}

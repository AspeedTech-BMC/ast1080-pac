#[doc = "Register `OTP_REG148` reader"]
pub type R = crate::R<OtpReg148Spec>;
#[doc = "Register `OTP_REG148` writer"]
pub type W = crate::W<OtpReg148Spec>;
#[doc = "Field `REGREGIONUSR1REN` reader - REG_REGION_USR1_REN"]
pub type Regregionusr1renR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR1REN` writer - REG_REGION_USR1_REN"]
pub type Regregionusr1renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR1WEN` reader - REG_REGION_USR1_WEN"]
pub type Regregionusr1wenR = crate::FieldReader;
#[doc = "Field `REGREGIONUSR1WEN` writer - REG_REGION_USR1_WEN"]
pub type Regregionusr1wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONUSR1LOCK` reader - REG_REGION_USR1_LOCK"]
pub type Regregionusr1lockR = crate::BitReader;
#[doc = "Field `REGREGIONUSR1LOCK` writer - REG_REGION_USR1_LOCK"]
pub type Regregionusr1lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_USR1_REN"]
    #[inline(always)]
    pub fn regregionusr1ren(&self) -> Regregionusr1renR {
        Regregionusr1renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR1_WEN"]
    #[inline(always)]
    pub fn regregionusr1wen(&self) -> Regregionusr1wenR {
        Regregionusr1wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_USR1_LOCK"]
    #[inline(always)]
    pub fn regregionusr1lock(&self) -> Regregionusr1lockR {
        Regregionusr1lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_USR1_REN"]
    #[inline(always)]
    pub fn regregionusr1ren(&mut self) -> Regregionusr1renW<OtpReg148Spec> {
        Regregionusr1renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_USR1_WEN"]
    #[inline(always)]
    pub fn regregionusr1wen(&mut self) -> Regregionusr1wenW<OtpReg148Spec> {
        Regregionusr1wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_USR1_LOCK"]
    #[inline(always)]
    pub fn regregionusr1lock(&mut self) -> Regregionusr1lockW<OtpReg148Spec> {
        Regregionusr1lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg148Spec;
impl crate::RegisterSpec for OtpReg148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg148::R`](R) reader structure"]
impl crate::Readable for OtpReg148Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg148::W`](W) writer structure"]
impl crate::Writable for OtpReg148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG148 to value 0xffff"]
impl crate::Resettable for OtpReg148Spec {
    const RESET_VALUE: u32 = 0xffff;
}

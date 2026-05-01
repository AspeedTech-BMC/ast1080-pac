#[doc = "Register `OTP_REG144` reader"]
pub type R = crate::R<OtpReg144Spec>;
#[doc = "Register `OTP_REG144` writer"]
pub type W = crate::W<OtpReg144Spec>;
#[doc = "Field `REGREGIONUSR0STARTOFFSET` reader - REG_REGION_USR0_START_OFFSET"]
pub type Regregionusr0startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR0STARTOFFSET` writer - REG_REGION_USR0_START_OFFSET"]
pub type Regregionusr0startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONUSR0SIZE` reader - REG_REGION_USR0_SIZE"]
pub type Regregionusr0sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR0SIZE` writer - REG_REGION_USR0_SIZE"]
pub type Regregionusr0sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_USR0_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr0startoffset(&self) -> Regregionusr0startoffsetR {
        Regregionusr0startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR0_SIZE"]
    #[inline(always)]
    pub fn regregionusr0size(&self) -> Regregionusr0sizeR {
        Regregionusr0sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_USR0_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr0startoffset(&mut self) -> Regregionusr0startoffsetW<OtpReg144Spec> {
        Regregionusr0startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR0_SIZE"]
    #[inline(always)]
    pub fn regregionusr0size(&mut self) -> Regregionusr0sizeW<OtpReg144Spec> {
        Regregionusr0sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg144Spec;
impl crate::RegisterSpec for OtpReg144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg144::R`](R) reader structure"]
impl crate::Readable for OtpReg144Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg144::W`](W) writer structure"]
impl crate::Writable for OtpReg144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG144 to value 0x0600_0000"]
impl crate::Resettable for OtpReg144Spec {
    const RESET_VALUE: u32 = 0x0600_0000;
}

#[doc = "Register `OTP_REG14C` reader"]
pub type R = crate::R<OtpReg14cSpec>;
#[doc = "Register `OTP_REG14C` writer"]
pub type W = crate::W<OtpReg14cSpec>;
#[doc = "Field `REGREGIONUSR1STARTOFFSET` reader - REG_REGION_USR1_START_OFFSET"]
pub type Regregionusr1startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR1STARTOFFSET` writer - REG_REGION_USR1_START_OFFSET"]
pub type Regregionusr1startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONUSR1SIZE` reader - REG_REGION_USR1_SIZE"]
pub type Regregionusr1sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR1SIZE` writer - REG_REGION_USR1_SIZE"]
pub type Regregionusr1sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_USR1_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr1startoffset(&self) -> Regregionusr1startoffsetR {
        Regregionusr1startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR1_SIZE"]
    #[inline(always)]
    pub fn regregionusr1size(&self) -> Regregionusr1sizeR {
        Regregionusr1sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_USR1_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr1startoffset(&mut self) -> Regregionusr1startoffsetW<OtpReg14cSpec> {
        Regregionusr1startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR1_SIZE"]
    #[inline(always)]
    pub fn regregionusr1size(&mut self) -> Regregionusr1sizeW<OtpReg14cSpec> {
        Regregionusr1sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_1\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg14cSpec;
impl crate::RegisterSpec for OtpReg14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg14c::R`](R) reader structure"]
impl crate::Readable for OtpReg14cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg14c::W`](W) writer structure"]
impl crate::Writable for OtpReg14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG14C to value 0x0600_0000"]
impl crate::Resettable for OtpReg14cSpec {
    const RESET_VALUE: u32 = 0x0600_0000;
}

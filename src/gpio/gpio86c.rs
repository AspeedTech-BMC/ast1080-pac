#[doc = "Register `GPIO86C` reader"]
pub type R = crate::R<Gpio86cSpec>;
#[doc = "Register `GPIO86C` writer"]
pub type W = crate::W<Gpio86cSpec>;
#[doc = "Field `GPIO092WrPrivilegeOfMaster` reader - GPIO092 Write Privilege of Master"]
pub type Gpio092wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO092WrPrivilegeOfMaster` writer - GPIO092 Write Privilege of Master"]
pub type Gpio092wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO093WrPrivilegeOfMaster` reader - GPIO093 Write Privilege of Master"]
pub type Gpio093wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO093WrPrivilegeOfMaster` writer - GPIO093 Write Privilege of Master"]
pub type Gpio093wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO094WrPrivilegeOfMaster` reader - GPIO094 Write Privilege of Master"]
pub type Gpio094wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO094WrPrivilegeOfMaster` writer - GPIO094 Write Privilege of Master"]
pub type Gpio094wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO095WrPrivilegeOfMaster` reader - GPIO095 Write Privilege of Master"]
pub type Gpio095wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO095WrPrivilegeOfMaster` writer - GPIO095 Write Privilege of Master"]
pub type Gpio095wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO092 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio092wr_privilege_of_master(&self) -> Gpio092wrPrivilegeOfMasterR {
        Gpio092wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO093 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio093wr_privilege_of_master(&self) -> Gpio093wrPrivilegeOfMasterR {
        Gpio093wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO094 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio094wr_privilege_of_master(&self) -> Gpio094wrPrivilegeOfMasterR {
        Gpio094wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO095 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio095wr_privilege_of_master(&self) -> Gpio095wrPrivilegeOfMasterR {
        Gpio095wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO092 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio092wr_privilege_of_master(&mut self) -> Gpio092wrPrivilegeOfMasterW<Gpio86cSpec> {
        Gpio092wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO093 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio093wr_privilege_of_master(&mut self) -> Gpio093wrPrivilegeOfMasterW<Gpio86cSpec> {
        Gpio093wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO094 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio094wr_privilege_of_master(&mut self) -> Gpio094wrPrivilegeOfMasterW<Gpio86cSpec> {
        Gpio094wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO095 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio095wr_privilege_of_master(&mut self) -> Gpio095wrPrivilegeOfMasterW<Gpio86cSpec> {
        Gpio095wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio86c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio86c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio86cSpec;
impl crate::RegisterSpec for Gpio86cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio86c::R`](R) reader structure"]
impl crate::Readable for Gpio86cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio86c::W`](W) writer structure"]
impl crate::Writable for Gpio86cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO86C to value 0xffff_ffff"]
impl crate::Resettable for Gpio86cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

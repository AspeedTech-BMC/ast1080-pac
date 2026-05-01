#[doc = "Register `GPIO96C` reader"]
pub type R = crate::R<Gpio96cSpec>;
#[doc = "Register `GPIO96C` writer"]
pub type W = crate::W<Gpio96cSpec>;
#[doc = "Field `GPIO092ReadPrivilegeOfMaster` reader - GPIO092 Read Privilege of Master"]
pub type Gpio092readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO092ReadPrivilegeOfMaster` writer - GPIO092 Read Privilege of Master"]
pub type Gpio092readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO093ReadPrivilegeOfMaster` reader - GPIO093 Read Privilege of Master"]
pub type Gpio093readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO093ReadPrivilegeOfMaster` writer - GPIO093 Read Privilege of Master"]
pub type Gpio093readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO094ReadPrivilegeOfMaster` reader - GPIO094 Read Privilege of Master"]
pub type Gpio094readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO094ReadPrivilegeOfMaster` writer - GPIO094 Read Privilege of Master"]
pub type Gpio094readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO095ReadPrivilegeOfMaster` reader - GPIO095 Read Privilege of Master"]
pub type Gpio095readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO095ReadPrivilegeOfMaster` writer - GPIO095 Read Privilege of Master"]
pub type Gpio095readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO092 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio092read_privilege_of_master(&self) -> Gpio092readPrivilegeOfMasterR {
        Gpio092readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO093 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio093read_privilege_of_master(&self) -> Gpio093readPrivilegeOfMasterR {
        Gpio093readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO094 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio094read_privilege_of_master(&self) -> Gpio094readPrivilegeOfMasterR {
        Gpio094readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO095 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio095read_privilege_of_master(&self) -> Gpio095readPrivilegeOfMasterR {
        Gpio095readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO092 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio092read_privilege_of_master(
        &mut self,
    ) -> Gpio092readPrivilegeOfMasterW<Gpio96cSpec> {
        Gpio092readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO093 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio093read_privilege_of_master(
        &mut self,
    ) -> Gpio093readPrivilegeOfMasterW<Gpio96cSpec> {
        Gpio093readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO094 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio094read_privilege_of_master(
        &mut self,
    ) -> Gpio094readPrivilegeOfMasterW<Gpio96cSpec> {
        Gpio094readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO095 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio095read_privilege_of_master(
        &mut self,
    ) -> Gpio095readPrivilegeOfMasterW<Gpio96cSpec> {
        Gpio095readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio96c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio96c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio96cSpec;
impl crate::RegisterSpec for Gpio96cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio96c::R`](R) reader structure"]
impl crate::Readable for Gpio96cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio96c::W`](W) writer structure"]
impl crate::Writable for Gpio96cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO96C to value 0xffff_ffff"]
impl crate::Resettable for Gpio96cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

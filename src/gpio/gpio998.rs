#[doc = "Register `GPIO998` reader"]
pub type R = crate::R<Gpio998Spec>;
#[doc = "Register `GPIO998` writer"]
pub type W = crate::W<Gpio998Spec>;
#[doc = "Field `GPIO136ReadPrivilegeOfMaster` reader - GPIO136 Read Privilege of Master"]
pub type Gpio136readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO136ReadPrivilegeOfMaster` writer - GPIO136 Read Privilege of Master"]
pub type Gpio136readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO137ReadPrivilegeOfMaster` reader - GPIO137 Read Privilege of Master"]
pub type Gpio137readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO137ReadPrivilegeOfMaster` writer - GPIO137 Read Privilege of Master"]
pub type Gpio137readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO138ReadPrivilegeOfMaster` reader - GPIO138 Read Privilege of Master"]
pub type Gpio138readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO138ReadPrivilegeOfMaster` writer - GPIO138 Read Privilege of Master"]
pub type Gpio138readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO139ReadPrivilegeOfMaster` reader - GPIO139 Read Privilege of Master"]
pub type Gpio139readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO139ReadPrivilegeOfMaster` writer - GPIO139 Read Privilege of Master"]
pub type Gpio139readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO136 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio136read_privilege_of_master(&self) -> Gpio136readPrivilegeOfMasterR {
        Gpio136readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO137 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio137read_privilege_of_master(&self) -> Gpio137readPrivilegeOfMasterR {
        Gpio137readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO138 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio138read_privilege_of_master(&self) -> Gpio138readPrivilegeOfMasterR {
        Gpio138readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO139 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio139read_privilege_of_master(&self) -> Gpio139readPrivilegeOfMasterR {
        Gpio139readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO136 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio136read_privilege_of_master(
        &mut self,
    ) -> Gpio136readPrivilegeOfMasterW<Gpio998Spec> {
        Gpio136readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO137 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio137read_privilege_of_master(
        &mut self,
    ) -> Gpio137readPrivilegeOfMasterW<Gpio998Spec> {
        Gpio137readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO138 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio138read_privilege_of_master(
        &mut self,
    ) -> Gpio138readPrivilegeOfMasterW<Gpio998Spec> {
        Gpio138readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO139 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio139read_privilege_of_master(
        &mut self,
    ) -> Gpio139readPrivilegeOfMasterW<Gpio998Spec> {
        Gpio139readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio998::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio998::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio998Spec;
impl crate::RegisterSpec for Gpio998Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio998::R`](R) reader structure"]
impl crate::Readable for Gpio998Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio998::W`](W) writer structure"]
impl crate::Writable for Gpio998Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO998 to value 0xffff_ffff"]
impl crate::Resettable for Gpio998Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
